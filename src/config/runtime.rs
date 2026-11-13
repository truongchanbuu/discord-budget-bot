use super::discord::DiscordConfig;
use crate::runtime::ApplicationError;
use std::ffi::OsString;

#[derive(Debug)]
pub(crate) enum RuntimeConfig {
    Offline,
    Discord(DiscordConfig),
}

impl RuntimeConfig {
    pub(crate) fn load(
        lookup: &mut impl FnMut(&str) -> Option<OsString>,
    ) -> Result<Self, ApplicationError> {
        match lookup("FUNDGUARD_MODE") {
            None => Ok(Self::Offline),
            Some(value) if value == "offline" => Ok(Self::Offline),
            Some(value) if value == "discord" => DiscordConfig::load(lookup).map(Self::Discord),
            _ => Err(ApplicationError::configuration("FUNDGUARD_MODE")),
        }
    }
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod tests;
