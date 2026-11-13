use sqlx::{
    PgPool, SqlSafeStr,
    migrate::{Migration, MigrationType, Migrator},
    postgres::PgPoolOptions,
};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
#[error("finance storage unavailable")]
pub(crate) struct DatabaseError;

pub(crate) async fn connect(url: &str) -> Result<PgPool, DatabaseError> {
    tokio::time::timeout(Duration::from_secs(15), async {
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .acquire_timeout(Duration::from_secs(3))
            .connect(url)
            .await
            .map_err(|_| DatabaseError)?;
        let migration = Migration::new(
            1,
            "personal journal".into(),
            MigrationType::Simple,
            include_str!("../../migrations/0001_personal_journal.sql").into_sql_str(),
            false,
        );
        if Migrator::with_migrations(vec![migration])
            .run(&pool)
            .await
            .is_err()
        {
            pool.close().await;
            return Err(DatabaseError);
        }
        Ok(pool)
    })
    .await
    .map_err(|_| DatabaseError)?
}
