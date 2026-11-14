use super::tests::fixture;
use sqlx::{Row, SqlSafeStr};

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn existing_v1_database_upgrades_without_losing_records() {
    use sqlx::{
        ConnectOptions,
        migrate::{Migration, MigrationType, Migrator},
    };
    use std::str::FromStr;
    let f = fixture().await;
    sqlx::query("CREATE DATABASE fundguard_test_upgrade")
        .execute(&f.pool)
        .await
        .unwrap();
    let options = sqlx::postgres::PgConnectOptions::from_str(
        &std::env::var("FUNDGUARD_TEST_DATABASE_URL").unwrap(),
    )
    .unwrap()
    .database("fundguard_test_upgrade");
    let url = options.to_url_lossy().to_string();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    Migrator::with_migrations(vec![Migration::new(
        1,
        "personal journal".into(),
        MigrationType::Simple,
        include_str!("../../../../migrations/0001_personal_journal.sql").into_sql_str(),
        false,
    )])
    .run(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO fg_owners(owner) VALUES('synthetic-upgrade')")
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
    for _ in 0..2 {
        let upgraded = crate::platform::database::connect(&url).await.unwrap();
        assert_eq!(
            sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM fg_owners WHERE owner='synthetic-upgrade'"
            )
            .fetch_one(&upgraded)
            .await
            .unwrap(),
            1
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM _sqlx_migrations WHERE success")
                .fetch_one(&upgraded)
                .await
                .unwrap(),
            4
        );
        upgraded.close().await;
    }
    sqlx::query("DROP DATABASE fundguard_test_upgrade")
        .execute(&f.pool)
        .await
        .unwrap();
    f.pool.close().await;
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn backend_tables_deny_data_api_roles_after_upgrade() {
    let f = fixture().await;
    f.preview("security:1", "nhận lương 1tr").await;
    let mut tx = f.pool.begin().await.unwrap();
    sqlx::raw_sql(
        "CREATE ROLE anon; CREATE ROLE authenticated; CREATE ROLE service_role BYPASSRLS;
         GRANT USAGE ON SCHEMA public TO anon, authenticated, service_role;
         GRANT ALL ON fg_owners,fg_funds,fg_drafts,fg_entries,_sqlx_migrations TO anon,authenticated,service_role;
         GRANT ALL ON SEQUENCE fg_drafts_id_seq,fg_entries_id_seq TO anon,authenticated,service_role;
         GRANT EXECUTE ON FUNCTION fg_immutable_entry() TO anon,authenticated,service_role;",
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    sqlx::raw_sql(include_str!("../../../../migrations/0002_backend_only.sql").into_sql_str())
        .execute(&mut *tx)
        .await
        .unwrap();
    let secured: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
         WHERE n.nspname='public' AND c.relname IN
         ('fg_owners','fg_funds','fg_drafts','fg_entries','_sqlx_migrations') AND c.relrowsecurity",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    assert_eq!(secured, 5);
    for role in ["anon", "authenticated", "service_role"] {
        for table in [
            "fg_owners",
            "fg_funds",
            "fg_drafts",
            "fg_entries",
            "_sqlx_migrations",
        ] {
            let row = sqlx::query(
                "SELECT has_table_privilege($1,$2,'SELECT') AS read,
                 has_table_privilege($1,$2,'INSERT,UPDATE,DELETE') AS write",
            )
            .bind(role)
            .bind(table)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
            assert!(!row.get::<bool, _>("read"));
            assert!(!row.get::<bool, _>("write"));
        }
        assert!(
            !sqlx::query_scalar::<_, bool>(
                "SELECT has_function_privilege($1,'fg_immutable_entry()','EXECUTE')",
            )
            .bind(role)
            .fetch_one(&mut *tx)
            .await
            .unwrap()
        );
        assert!(
            !sqlx::query_scalar::<_, bool>(
                "SELECT has_sequence_privilege($1,'fg_entries_id_seq','USAGE,SELECT,UPDATE')",
            )
            .bind(role)
            .fetch_one(&mut *tx)
            .await
            .unwrap()
        );
    }
    sqlx::raw_sql("GRANT SELECT ON fg_drafts TO anon; SET LOCAL ROLE anon;")
        .execute(&mut *tx)
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM fg_drafts")
            .fetch_one(&mut *tx)
            .await
            .unwrap(),
        0
    );
    sqlx::query("RESET ROLE").execute(&mut *tx).await.unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM _sqlx_migrations WHERE success")
            .fetch_one(&f.pool)
            .await
            .unwrap(),
        4
    );
    assert!(
        f.confirm(&f.preview("security:2", "nhận lương 1tr").await)
            .await
            .is_ok()
    );
    f.pool.close().await;
}
