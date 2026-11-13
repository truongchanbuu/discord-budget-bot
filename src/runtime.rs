use crate::StartupError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{kind}")]
pub struct ApplicationError {
    kind: ErrorKind,
}

#[derive(Debug, Error)]
enum ErrorKind {
    #[error("{0} is missing or invalid")]
    Configuration(&'static str),
    #[error("{0}")]
    Initialization(StartupError),
    #[error("Discord session failed: {0}")]
    Session(&'static str),
}

impl ApplicationError {
    pub fn event_name(&self) -> &'static str {
        match self.kind {
            ErrorKind::Session(_) => "discord_session_failed",
            _ => "application_initialization_failed",
        }
    }

    pub(crate) fn configuration(setting: &'static str) -> Self {
        Self {
            kind: ErrorKind::Configuration(setting),
        }
    }

    pub(crate) fn session(category: &'static str) -> Self {
        Self {
            kind: ErrorKind::Session(category),
        }
    }
}

async fn dispatch<F>(
    lookup: &mut impl FnMut(&str) -> Option<std::ffi::OsString>,
    initialize: impl FnOnce() -> Result<(), StartupError>,
    connect: impl FnOnce(crate::config::discord::DiscordConfig) -> F,
) -> Result<(), ApplicationError>
where
    F: std::future::Future<Output = Result<(), ApplicationError>>,
{
    let config = crate::config::runtime::RuntimeConfig::load(lookup)?;
    initialize().map_err(|error| ApplicationError {
        kind: ErrorKind::Initialization(error),
    })?;
    match config {
        crate::config::runtime::RuntimeConfig::Offline => Ok(()),
        crate::config::runtime::RuntimeConfig::Discord(config) => connect(config).await,
    }
}

pub async fn start() -> Result<(), ApplicationError> {
    dispatch(
        &mut |name| std::env::var_os(name),
        crate::run,
        |config| async {
            let pool = match &config.database_url {
                None => None,
                Some(url) => Some(
                    crate::platform::database::connect(url.as_str())
                        .await
                        .map_err(|_| ApplicationError::session("finance_storage_unavailable"))?,
                ),
            };
            crate::platform::discord::run(config, pool)
                .await
                .map_err(|error| ApplicationError::session(error.category()))
        },
    )
    .await
}

#[cfg(test)]
mod tests;
