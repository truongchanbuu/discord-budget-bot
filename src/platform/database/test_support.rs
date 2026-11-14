use super::{DatabaseError, PgConnectOptions, PgPool};
use std::str::FromStr;

fn fixture_options(marker: &str, url: &str) -> Result<PgConnectOptions, DatabaseError> {
    if marker != "1" {
        return Err(DatabaseError);
    }
    let options = PgConnectOptions::from_str(url).map_err(|_| DatabaseError)?;
    let private_host = options.get_host().starts_with("/tmp/fundguard")
        || ["localhost", "127.0.0.1"].contains(&options.get_host());
    let private_database = options
        .get_database()
        .unwrap_or("")
        .starts_with("fundguard_test");
    if !private_host || !private_database {
        return Err(DatabaseError);
    }
    Ok(options)
}

pub(crate) async fn connect() -> PgPool {
    let marker = std::env::var("FUNDGUARD_TEST_PRIVATE_CLUSTER").unwrap_or_default();
    let url = std::env::var("FUNDGUARD_TEST_DATABASE_URL").unwrap_or_default();
    fixture_options(&marker, &url).expect("disposable private PostgreSQL fixture required");
    super::connect(&url)
        .await
        .expect("private fixture initialization failed")
}

#[test]
fn guard_rejects_remote_missing_marker_and_nonfixture_database() {
    for (marker, url) in [
        ("", "postgresql://localhost/fundguard_test"),
        ("0", "postgresql://localhost/fundguard_test"),
        ("1", "postgresql://remote.example/fundguard_test"),
        ("1", "postgresql://localhost/production"),
        ("1", "invalid-secret-marker"),
    ] {
        assert!(fixture_options(marker, url).is_err());
    }
    for url in [
        "postgresql://localhost/fundguard_test",
        "postgresql://127.0.0.1/fundguard_test_isolated",
        "postgresql://localhost/fundguard_test?host=/tmp/fundguard-test/socket",
    ] {
        assert!(fixture_options("1", url).is_ok());
    }
}
