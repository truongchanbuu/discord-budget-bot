use super::*;

#[test]
fn session_categories_have_no_provider_payloads_or_sources() {
    for error in [
        SessionError::Build,
        SessionError::StartupTimeout,
        SessionError::Registration,
        SessionError::RegistrationTimeout,
        SessionError::Setup,
        SessionError::Connection,
        SessionError::RecoveryTimeout,
        SessionError::ShutdownTimeout,
        SessionError::Shutdown,
        SessionError::Signal,
        SessionError::Task,
    ] {
        assert!(!error.category().is_empty());
        assert!(!format!("{error} {error:#?}").contains("provider-marker"));
        assert!(std::error::Error::source(&error).is_none());
    }
}
