use crate::features::identity::presentation::{CommandData, ReadinessError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("Discord session unavailable")]
pub(crate) enum SessionError {
    Build,
    StartupTimeout,
    Registration,
    RegistrationTimeout,
    Setup,
    Connection,
    RecoveryTimeout,
    ShutdownTimeout,
    Shutdown,
    Signal,
    Task,
}

impl SessionError {
    pub(crate) fn category(self) -> &'static str {
        match self {
            Self::Build => "client_build_failed",
            Self::StartupTimeout => "startup_timeout",
            Self::Registration => "registration_failed",
            Self::RegistrationTimeout => "registration_timeout",
            Self::Setup => "setup_failed",
            Self::Connection => "connection_failed",
            Self::RecoveryTimeout => "recovery_timeout",
            Self::ShutdownTimeout => "shutdown_timeout",
            Self::Shutdown => "shutdown_failed",
            Self::Signal => "signal_failed",
            Self::Task => "client_task_failed",
        }
    }
}

pub(super) async fn on_error(error: poise::FrameworkError<'_, CommandData, ReadinessError>) {
    let category = match error {
        poise::FrameworkError::Setup { .. } => "setup_failed",
        poise::FrameworkError::Command { .. } => "response_failed",
        _ => "framework_failed",
    };
    tracing::warn!(event = "discord_interaction_failed", category);
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;
