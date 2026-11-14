use crate::platform::database::test_support;

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn private_schema_constraints_and_security_survive_reconnect() {
    let pool = test_support::connect().await;
    let secured: i64=sqlx::query_scalar("SELECT count(*) FROM pg_class WHERE relname IN ('fg_finance_spaces','fg_space_operations','fg_space_viewers','fg_space_access_events') AND relrowsecurity")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(secured, 4);
    for table in [
        "fg_finance_spaces",
        "fg_space_operations",
        "fg_space_viewers",
        "fg_space_access_events",
    ] {
        for role in ["anon", "authenticated", "service_role"] {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT FROM pg_roles WHERE rolname=$1)")
                    .bind(role)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            if exists {
                let visible: bool = sqlx::query_scalar(
                    "SELECT has_table_privilege($1,$2,'SELECT,INSERT,UPDATE,DELETE')",
                )
                .bind(role)
                .bind(table)
                .fetch_one(&pool)
                .await
                .unwrap();
                assert!(!visible);
            }
        }
    }
    pool.close().await;
    let again = test_support::connect().await;
    let migrations: i64 = sqlx::query_scalar("SELECT count(*) FROM _sqlx_migrations WHERE success")
        .fetch_one(&again)
        .await
        .unwrap();
    assert_eq!(migrations, 4);
}

async fn seed(pool: &sqlx::PgPool) -> (i64, String) {
    let owner: String = sqlx::query_scalar(
        "INSERT INTO fg_owners(owner) VALUES(gen_random_uuid()::text) RETURNING owner",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let id:i64=sqlx::query_scalar("INSERT INTO fg_finance_spaces(guild_id,member_id,journal_owner,generation,category_id,state) VALUES('10',(nextval('fg_finance_spaces_id_seq')+100)::text,$1,gen_random_uuid()::text,'13','prepared') RETURNING id").bind(&owner).fetch_one(pool).await.unwrap();
    (id, owner)
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn ownership_pending_operation_and_audit_constraints_are_enforced() {
    let pool = test_support::connect().await;
    let (id, owner) = seed(&pool).await;
    for sql in [
        "UPDATE fg_finance_spaces SET journal_owner='foreign' WHERE id=$1",
        "UPDATE fg_finance_spaces SET member_id='999' WHERE id=$1",
        "UPDATE fg_finance_spaces SET guild_id='999' WHERE id=$1",
        "UPDATE fg_finance_spaces SET revision=-1 WHERE id=$1",
        "UPDATE fg_finance_spaces SET state='ready' WHERE id=$1",
        "UPDATE fg_finance_spaces SET channel_id='01' WHERE id=$1",
    ] {
        assert!(sqlx::query(sql).bind(id).execute(&pool).await.is_err());
    }
    let operation:String=sqlx::query_scalar("INSERT INTO fg_space_operations(space_id,kind,source,actor,generation,expected_revision,phase) SELECT id,'create','test:create','11',generation,revision,'prepared' FROM fg_finance_spaces WHERE id=$1 RETURNING id")
        .bind(id).fetch_one(&pool).await.unwrap();
    for statement in [
        "UPDATE fg_space_operations SET source='tampered' WHERE id=$1",
        "UPDATE fg_space_operations SET actor='22' WHERE id=$1",
        "UPDATE fg_space_operations SET generation=gen_random_uuid()::text WHERE id=$1",
        "UPDATE fg_space_operations SET expected_revision=expected_revision+1 WHERE id=$1",
    ] {
        assert!(
            sqlx::query(statement)
                .bind(&operation)
                .execute(&pool)
                .await
                .is_err()
        );
    }
    assert!(sqlx::query("INSERT INTO fg_space_operations(space_id,kind,source,actor,generation,expected_revision,phase) SELECT id,'create','test:duplicate','11',generation,revision,'prepared' FROM fg_finance_spaces WHERE id=$1").bind(id).execute(&pool).await.is_err());
    sqlx::query("INSERT INTO fg_space_access_events(space_id,operation_id,source,actor,action,outcome,transition_revision) VALUES($1,$2,'test:create','11','reserved','pending',0)")
        .bind(id).bind(&operation).execute(&pool).await.unwrap();
    assert!(
        sqlx::query("UPDATE fg_space_access_events SET action='verified' WHERE space_id=$1")
            .bind(id)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM fg_space_access_events WHERE space_id=$1")
            .bind(id)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(sqlx::query("INSERT INTO fg_space_access_events(space_id,operation_id,source,actor,action,outcome,transition_revision) VALUES($1,$2,'test:create','11','reserved','pending',0)").bind(id).bind(operation).execute(&pool).await.is_err());
    let unchanged: String =
        sqlx::query_scalar("SELECT journal_owner FROM fg_finance_spaces WHERE id=$1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(unchanged, owner);
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn incarnation_columns_require_canonical_valid_utc_nanoseconds() {
    let pool = test_support::connect().await;
    let (id, _) = seed(&pool).await;
    for invalid in [
        "secret-marker",
        "2026-02-30T00:00:00.000000000Z",
        "2026-11-14T00:00:00.123456Z",
        "2026-11-14T00:00:00.1234567890Z",
        "2026-11-14T07:00:00.123456789+07:00",
    ] {
        assert!(
            sqlx::query("UPDATE fg_finance_spaces SET owner_joined_at=$1 WHERE id=$2")
                .bind(invalid)
                .bind(id)
                .execute(&pool)
                .await
                .is_err()
        );
    }
    let expected = "2026-11-14T00:00:00.123456789Z";
    sqlx::query("UPDATE fg_finance_spaces SET owner_joined_at=$1 WHERE id=$2")
        .bind(expected)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    let stored: String =
        sqlx::query_scalar("SELECT owner_joined_at FROM fg_finance_spaces WHERE id=$1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(stored, expected);
}
