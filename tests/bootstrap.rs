mod support;

use std::ffi::{OsStr, OsString};
use std::process::Command;

fn execute(level: Option<&OsStr>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_fundguard"));
    command.env_clear();
    if let Some(level) = level {
        command.env("FUNDGUARD_LOG_LEVEL", level);
    }
    support::run(&mut command)
}

#[test]
fn default_and_supported_levels_initialize_without_services() {
    for (level, logged) in [
        (None, true),
        (Some("trace"), true),
        (Some("debug"), true),
        (Some("info"), true),
        (Some("warn"), false),
        (Some("error"), false),
    ] {
        let output = execute(level.map(OsStr::new));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "level {level:?} failed");
        assert!(output.stdout.is_empty());
        assert_eq!(stderr.contains("application_initialized"), logged);
        assert!(!stderr.contains('\u{1b}'));
    }
}

fn assert_invalid(level: &OsStr) {
    let output = execute(Some(level));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(stderr.contains("ERROR"));
    assert!(stderr.contains("event="));
    assert!(stderr.contains("application_initialization_failed"));
    assert!(stderr.contains("FUNDGUARD_LOG_LEVEL"));
    assert!(stderr.contains("trace, debug, info, warn, error"));
    assert!(!stderr.contains("application_initialized"));
    assert!(!stderr.contains("invalid-foundation-marker"));
    assert!(!stderr.contains('\u{1b}'));
}

#[test]
fn invalid_configuration_uses_redacted_structured_failure() {
    for input in ["", "INFO", " info", "info ", "invalid-foundation-marker"] {
        assert_invalid(&OsString::from(input));
    }
}

#[cfg(unix)]
#[test]
fn non_unicode_configuration_fails_safely() {
    use std::os::unix::ffi::OsStringExt;
    assert_invalid(&OsString::from_vec(vec![0xff, 0xfe]));
}

#[test]
fn ambient_credentials_do_not_activate_integrations_or_leak() {
    for level in ["info", "invalid-foundation-marker"] {
        let output = support::run(
            Command::new(env!("CARGO_BIN_EXE_fundguard"))
                .env_clear()
                .env("FUNDGUARD_LOG_LEVEL", level)
                .env("DISCORD_TOKEN", "foundation-token-marker")
                .env("DATABASE_URL", "foundation-database-marker"),
        );
        assert_eq!(output.status.success(), level == "info");
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        for marker in [
            "foundation-token-marker",
            "foundation-database-marker",
            "invalid-foundation-marker",
        ] {
            assert!(!stderr.contains(marker));
        }
    }
}
