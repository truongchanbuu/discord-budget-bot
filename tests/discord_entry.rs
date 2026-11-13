mod support;
use std::{ffi::OsString, process::Command};

fn execute(mode: Option<&str>, extra: &[(&str, OsString)]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_fundguard"));
    command.env_clear();
    if let Some(mode) = mode {
        command.env("FUNDGUARD_MODE", mode);
    }
    for (key, value) in extra {
        command.env(key, value);
    }
    support::run(&mut command)
}

#[test]
fn offline_ignores_live_fields_and_exits_without_services() {
    for mode in [None, Some("offline")] {
        let output = execute(
            mode,
            &[
                ("FUNDGUARD_DISCORD_TOKEN", "synthetic-token-marker".into()),
                ("FUNDGUARD_DISCORD_GUILD_ID", "invalid-guild-marker".into()),
            ],
        );
        assert!(output.status.success());
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("application_initialized"));
        assert!(!stderr.contains("discord_ready"));
        assert!(!stderr.contains("marker"));
    }
}

#[test]
fn invalid_mode_and_incomplete_live_configuration_fail_before_provider_access() {
    for mode in ["", "invalid-mode-marker", "discord"] {
        let output = execute(Some(mode), &[]);
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("application_initialization_failed"));
        assert!(!stderr.contains("application_initialized"));
        assert!(!stderr.contains("discord_ready"));
        assert!(!stderr.contains("marker"));
    }
}

#[test]
fn every_incomplete_live_field_is_reported_without_provider_calls() {
    let values: Vec<_> = [
        ("FUNDGUARD_DISCORD_TOKEN", "synthetic-token-marker"),
        ("FUNDGUARD_DISCORD_GUILD_ID", "12"),
        ("FUNDGUARD_DISCORD_OWNER_ID", "34"),
        ("FUNDGUARD_OWNER_REF", "owner-marker"),
        ("FUNDGUARD_OWNER_STATE", "enabled"),
    ]
    .into_iter()
    .map(|(key, value)| (key, OsString::from(value)))
    .collect();
    for missing in 0..values.len() {
        let mut settings = values.clone();
        settings.remove(missing);
        let output = execute(Some("discord"), &settings);
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(values[missing].0));
        assert!(!stderr.contains("marker"));
        assert!(!stderr.contains("discord_ready"));
    }
}

#[cfg(unix)]
#[test]
fn nontext_live_fields_are_ignored_offline_and_rejected_when_selected() {
    use std::os::unix::ffi::OsStringExt;
    for mode in ["offline", "discord"] {
        let output = execute(
            Some(mode),
            &[("FUNDGUARD_DISCORD_TOKEN", OsString::from_vec(vec![255]))],
        );
        assert_eq!(output.status.success(), mode == "offline");
    }
}

#[test]
fn invalid_logging_prevents_a_fully_configured_connection() {
    let output = execute(
        Some("discord"),
        &[
            ("FUNDGUARD_DISCORD_TOKEN", "synthetic-token-marker".into()),
            ("FUNDGUARD_DISCORD_GUILD_ID", "12".into()),
            ("FUNDGUARD_DISCORD_OWNER_ID", "34".into()),
            ("FUNDGUARD_OWNER_REF", "owner-marker".into()),
            ("FUNDGUARD_OWNER_STATE", "enabled".into()),
            ("FUNDGUARD_LOG_LEVEL", "invalid-log-marker".into()),
        ],
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("FUNDGUARD_LOG_LEVEL"));
    assert!(stderr.contains("application_initialization_failed"));
    assert!(!stderr.contains("application_initialized"));
    assert!(!stderr.contains("marker"));
}

#[test]
fn ambient_rust_log_cannot_override_primary_or_fallback_policy() {
    for (level, rust_log, success, initialized) in [
        ("info", "off", true, true),
        ("error", "trace", true, false),
        ("invalid-log-marker", "off", false, false),
    ] {
        let output = execute(
            Some("offline"),
            &[
                ("FUNDGUARD_LOG_LEVEL", level.into()),
                ("RUST_LOG", rust_log.into()),
            ],
        );
        assert_eq!(output.status.success(), success);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(stderr.contains("application_initialized"), initialized);
        assert_eq!(
            stderr.contains("application_initialization_failed"),
            !success
        );
        assert!(!stderr.contains("invalid-log-marker"));
    }
}
