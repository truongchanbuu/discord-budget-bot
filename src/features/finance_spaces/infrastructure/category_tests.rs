use super::category::*;
use crate::{
    features::finance_spaces::{ChannelRef, ServerRef},
    platform::database::test_support,
};

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn category_reservation_is_exclusive_durable_and_backend_only() {
    let pool = test_support::connect().await;
    let guild = ServerRef::parse("999001").unwrap();
    let mut attempts = Vec::new();
    for _ in 0..10 {
        let pool = pool.clone();
        attempts.push(tokio::spawn(
            async move { reserve_category(&pool, guild).await },
        ));
    }
    let mut creates = 0;
    for attempt in attempts {
        match attempt.await.unwrap() {
            Ok(CategoryReservation::Create) => creates += 1,
            Err(crate::features::finance_spaces::domain::SpaceError::Uncertain) => {}
            _ => panic!("unexpected reservation outcome"),
        }
    }
    assert_eq!(creates, 1);
    assert!(reserve_category(&pool, guild).await.is_err());
    let channel = ChannelRef::parse("999002").unwrap();
    bind_category(&pool, guild, channel).await.unwrap();
    assert!(
        bind_category(&pool, guild, ChannelRef::parse("999003").unwrap())
            .await
            .is_err()
    );
    pool.close().await;
    let pool = test_support::connect().await;
    assert!(
        matches!(reserve_category(&pool, guild).await.unwrap(), CategoryReservation::Existing(id) if id == channel)
    );
    let secured: bool = sqlx::query_scalar(
        "SELECT relrowsecurity FROM pg_class WHERE oid='fg_finance_categories'::regclass",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(secured);
    let mut tx = pool.begin().await.unwrap();
    sqlx::raw_sql(sqlx::AssertSqlSafe("DO $$ BEGIN
        IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname='anon') THEN CREATE ROLE anon; END IF;
        IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname='authenticated') THEN CREATE ROLE authenticated; END IF;
        IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname='service_role') THEN CREATE ROLE service_role; END IF;
        END $$;
        GRANT ALL ON fg_finance_categories TO anon,authenticated,service_role;"))
        .execute(&mut *tx).await.unwrap();
    let migration = include_str!("../../../../migrations/0004_finance_category.sql");
    let revoke = &migration[migration.find("DO $$").unwrap()..];
    sqlx::raw_sql(sqlx::AssertSqlSafe(revoke))
        .execute(&mut *tx)
        .await
        .unwrap();
    for role in ["anon", "authenticated", "service_role"] {
        let denied: bool = sqlx::query_scalar("SELECT NOT has_table_privilege($1,'fg_finance_categories','SELECT,INSERT,UPDATE,DELETE')")
            .bind(role).fetch_one(&mut *tx).await.unwrap();
        assert!(denied);
    }
    tx.rollback().await.unwrap();
    assert!(
        sqlx::query("INSERT INTO fg_finance_categories(guild_id) VALUES('01')")
            .execute(&pool)
            .await
            .is_err()
    );
    pool.close().await;
}
