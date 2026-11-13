use crate::{
    features::{
        financial_core::OwnerRef,
        identity::{BindingState, DiscordAccountId, OwnerBinding},
    },
    runtime::ApplicationError,
};
use std::{ffi::OsString, fmt, num::NonZeroU64};

pub(crate) struct SecretToken(String);

impl SecretToken {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretToken([redacted])")
    }
}

pub(crate) struct DiscordConfig {
    pub(crate) token: SecretToken,
    pub(crate) guild: NonZeroU64,
    pub(crate) binding: OwnerBinding,
}

impl fmt::Debug for DiscordConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DiscordConfig([redacted])")
    }
}

fn required(
    lookup: &mut impl FnMut(&str) -> Option<OsString>,
    name: &'static str,
) -> Result<String, ApplicationError> {
    lookup(name)
        .and_then(|value| value.into_string().ok())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApplicationError::configuration(name))
}

impl DiscordConfig {
    pub(crate) fn load(
        lookup: &mut impl FnMut(&str) -> Option<OsString>,
    ) -> Result<Self, ApplicationError> {
        let token = required(lookup, "FUNDGUARD_DISCORD_TOKEN")?;
        if token.trim() != token || token.trim().is_empty() {
            return Err(ApplicationError::configuration("FUNDGUARD_DISCORD_TOKEN"));
        }
        let guild = required(lookup, "FUNDGUARD_DISCORD_GUILD_ID")?;
        if guild.starts_with('0')
            || guild.len() > 20
            || !guild.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(ApplicationError::configuration(
                "FUNDGUARD_DISCORD_GUILD_ID",
            ));
        }
        let guild = guild
            .parse::<u64>()
            .ok()
            .and_then(NonZeroU64::new)
            .ok_or_else(|| ApplicationError::configuration("FUNDGUARD_DISCORD_GUILD_ID"))?;
        let account = DiscordAccountId::parse(&required(lookup, "FUNDGUARD_DISCORD_OWNER_ID")?)
            .map_err(|_| ApplicationError::configuration("FUNDGUARD_DISCORD_OWNER_ID"))?;
        let owner = OwnerRef::new(required(lookup, "FUNDGUARD_OWNER_REF")?)
            .map_err(|_| ApplicationError::configuration("FUNDGUARD_OWNER_REF"))?;
        let state = match required(lookup, "FUNDGUARD_OWNER_STATE")?.as_str() {
            "enabled" => BindingState::Enabled,
            "disabled" => BindingState::Disabled,
            _ => return Err(ApplicationError::configuration("FUNDGUARD_OWNER_STATE")),
        };
        Ok(Self {
            token: SecretToken(token),
            guild,
            binding: OwnerBinding::new(account, owner, state),
        })
    }
}

#[cfg(test)]
#[path = "discord_tests.rs"]
mod tests;
