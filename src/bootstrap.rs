use crate::config::{Config, ConfigError};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{kind}")]
pub struct StartupError {
    kind: StartupErrorKind,
}

#[derive(Debug, Error)]
enum StartupErrorKind {
    #[error(transparent)]
    Configuration(ConfigError),
    #[error("application diagnostics could not be initialized")]
    DiagnosticsUnavailable,
}

pub fn run() -> Result<(), StartupError> {
    let config = Config::from_env().map_err(|error| StartupError {
        kind: StartupErrorKind::Configuration(error),
    })?;
    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .with_writer(std::io::stderr)
        .try_init()
        .map_err(|_| StartupError {
            kind: StartupErrorKind::DiagnosticsUnavailable,
        })?;
    tracing::info!(event = "application_initialized");
    Ok(())
}

#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod process;

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn subscriber_collision() {
        if std::env::var_os("FUNDGUARD_COLLISION_CHILD").is_some() {
            tracing::subscriber::set_global_default(
                tracing_subscriber::fmt()
                    .with_writer(std::io::sink)
                    .finish(),
            )
            .expect("install collision fixture");
            let error = run().expect_err("existing subscriber must fail startup");
            assert!(matches!(
                error.kind,
                StartupErrorKind::DiagnosticsUnavailable
            ));
            assert_eq!(
                error.to_string(),
                "application diagnostics could not be initialized"
            );
            return;
        }
        let output = process::run(
            Command::new(std::env::current_exe().expect("unit test executable"))
                .args([
                    "--exact",
                    "bootstrap::tests::subscriber_collision",
                    "--nocapture",
                ])
                .env_clear()
                .env("FUNDGUARD_COLLISION_CHILD", "1"),
        );
        assert!(output.status.success());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("application_initialized"));
    }

    #[test]
    fn configuration_error_identifies_the_setting() {
        let error = StartupError {
            kind: StartupErrorKind::Configuration(crate::config::ConfigError),
        };
        assert!(error.to_string().contains("FUNDGUARD_LOG_LEVEL"));
        assert!(
            error
                .to_string()
                .contains("trace, debug, info, warn, error")
        );
    }
}
