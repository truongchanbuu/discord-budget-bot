use super::*;
use sqlx::postgres::PgSslMode;

#[test]
fn remote_connections_require_encryption_and_preserve_verification() {
    let url = "postgresql://synthetic:marker@db.example.supabase.co:5432/postgres";
    assert!(matches!(
        options(url).unwrap().get_ssl_mode(),
        PgSslMode::Require
    ));
    for mode in ["disable", "allow"] {
        assert!(options(&format!("{url}?sslmode={mode}")).is_err());
    }
    assert!(matches!(
        options(&format!("{url}?sslmode=verify-full"))
            .unwrap()
            .get_ssl_mode(),
        PgSslMode::VerifyFull
    ));
    assert_eq!(
        format!("{}", options("invalid-marker").unwrap_err()),
        "finance storage unavailable"
    );
}

#[test]
fn session_pooler_is_supported_and_transaction_pooler_is_rejected() {
    assert!(options("postgresql://synthetic:marker@db.example.supabase.co:6543/postgres").is_err());
    assert!(
        options(
            "postgresql://postgres.synthetic:marker@aws-0-example.pooler.supabase.com:5432/postgres"
        )
        .is_ok()
    );
    assert!(
        options(
            "postgresql://postgres.synthetic:marker@aws-0-example.pooler.supabase.com:6543/postgres"
        )
        .is_err()
    );
    assert!(
        options(
            "postgresql://postgres.synthetic:marker@AWS-0-EXAMPLE.POOLER.SUPABASE.COM:6543/postgres"
        )
        .is_err()
    );
}

#[test]
fn local_socket_fixtures_remain_credential_free() {
    assert!(options("postgresql://synthetic@localhost/fundguard_test?sslmode=disable").is_ok());
    assert!(
        options("postgresql://synthetic@localhost/fundguard_test?host=/tmp/fundguard-test/socket")
            .is_ok()
    );
}
