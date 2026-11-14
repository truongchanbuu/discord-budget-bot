use sqlx::{
    PgPool, SqlSafeStr,
    migrate::{Migration, MigrationType, Migrator},
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
};
use std::{str::FromStr, time::Duration};

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
#[error("finance storage unavailable")]
pub(crate) struct DatabaseError;

fn options(url: &str) -> Result<PgConnectOptions, DatabaseError> {
    let options = PgConnectOptions::from_str(url).map_err(|_| DatabaseError)?;
    let host = options.get_host().to_ascii_lowercase();
    if (host.ends_with(".supabase.com") || host.ends_with(".supabase.co"))
        && options.get_port() == 6543
    {
        return Err(DatabaseError);
    }
    let local = host.starts_with('/') || matches!(host.as_str(), "localhost" | "127.0.0.1" | "::1");
    if local {
        return Ok(options);
    }
    match options.get_ssl_mode() {
        PgSslMode::Disable | PgSslMode::Allow => Err(DatabaseError),
        PgSslMode::Prefer => Ok(options.ssl_mode(PgSslMode::Require)),
        _ => Ok(options),
    }
}

pub(crate) async fn connect(url: &str) -> Result<PgPool, DatabaseError> {
    let options = options(url)?;
    tokio::time::timeout(Duration::from_secs(15), async {
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .acquire_timeout(Duration::from_secs(3))
            .connect_with(options)
            .await
            .map_err(|_| DatabaseError)?;
        let migration = Migration::new(
            1,
            "personal journal".into(),
            MigrationType::Simple,
            include_str!("../../migrations/0001_personal_journal.sql").into_sql_str(),
            false,
        );
        let security = Migration::new(
            2,
            "backend only".into(),
            MigrationType::Simple,
            include_str!("../../migrations/0002_backend_only.sql").into_sql_str(),
            false,
        );
        if Migrator::with_migrations(vec![migration, security])
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
