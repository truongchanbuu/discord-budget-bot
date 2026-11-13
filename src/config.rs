pub(crate) mod discord;
pub(crate) mod runtime;

use std::ffi::OsString;
use thiserror::Error;
use tracing::Level;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) log_level: Level,
}

#[derive(Debug, Error)]
#[error("FUNDGUARD_LOG_LEVEL must be one of: trace, debug, info, warn, error")]
pub(crate) struct ConfigError;

impl Config {
    pub(crate) fn from_env() -> Result<Self, ConfigError> {
        Self::parse(std::env::var_os("FUNDGUARD_LOG_LEVEL"))
    }

    fn parse(value: Option<OsString>) -> Result<Self, ConfigError> {
        let Some(value) = value else {
            return Ok(Self {
                log_level: Level::INFO,
            });
        };
        let log_level = match value.to_str() {
            Some("trace") => Level::TRACE,
            Some("debug") => Level::DEBUG,
            Some("info") => Level::INFO,
            Some("warn") => Level::WARN,
            Some("error") => Level::ERROR,
            _ => return Err(ConfigError),
        };
        Ok(Self { log_level })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use tracing::Level;

    #[test]
    fn absent_level_defaults_to_info() {
        assert_eq!(Config::parse(None).unwrap().log_level, Level::INFO);
    }

    #[test]
    fn supported_levels_are_typed() {
        for (input, expected) in [
            ("trace", Level::TRACE),
            ("debug", Level::DEBUG),
            ("info", Level::INFO),
            ("warn", Level::WARN),
            ("error", Level::ERROR),
        ] {
            assert_eq!(
                Config::parse(Some(OsString::from(input)))
                    .unwrap()
                    .log_level,
                expected
            );
        }
    }

    #[test]
    fn invalid_values_are_rejected_without_disclosure() {
        for input in ["", "INFO", " info", "info ", "invalid-foundation-marker"] {
            let error = Config::parse(Some(OsString::from(input))).unwrap_err();
            assert!(error.to_string().contains("FUNDGUARD_LOG_LEVEL"));
            assert!(
                error
                    .to_string()
                    .contains("trace, debug, info, warn, error")
            );
            assert!(!format!("{error} {error:?}").contains("invalid-foundation-marker"));
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_values_are_rejected_without_disclosure() {
        use std::os::unix::ffi::OsStringExt;
        let input = OsString::from_vec(vec![0xff, 0xfe]);
        assert!(Config::parse(Some(input)).is_err());
    }
}
