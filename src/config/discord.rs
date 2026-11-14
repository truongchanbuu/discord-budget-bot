use crate::{
    features::{
        financial_core::OwnerRef,
        identity::{BindingState, DiscordAccountId, OwnerBinding},
    },
    runtime::ApplicationError,
};
use std::{ffi::OsString, fmt, num::NonZeroU64};

pub(crate) struct SecretValue(String);

impl SecretValue {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretValue([redacted])")
    }
}

pub(crate) struct SpaceConfig {
    pub(crate) category: Option<NonZeroU64>,
    pub(crate) onboarding: Vec<NonZeroU64>,
}

impl fmt::Debug for SpaceConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SpaceConfig([redacted])")
    }
}

pub(crate) struct DiscordConfig {
    pub(crate) token: SecretValue,
    pub(crate) guild: NonZeroU64,
    pub(crate) binding: OwnerBinding,
    pub(crate) database_url: Option<SecretValue>,
    pub(crate) spaces: Option<SpaceConfig>,
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
        let database_url = match lookup("FUNDGUARD_FINANCE_DATABASE_URL") {
            None => None,
            Some(value) => {
                let value = value.into_string().map_err(|_| {
                    ApplicationError::configuration("FUNDGUARD_FINANCE_DATABASE_URL")
                })?;
                if value.is_empty() || value.trim() != value {
                    return Err(ApplicationError::configuration(
                        "FUNDGUARD_FINANCE_DATABASE_URL",
                    ));
                }
                Some(SecretValue(value))
            }
        };
        let spaces = load_spaces(lookup, database_url.is_some())?;
        Ok(Self {
            spaces,
            token: SecretValue(token),
            database_url,
            guild,
            binding: OwnerBinding::new(account, owner, state),
        })
    }
}

fn load_spaces(
    lookup: &mut impl FnMut(&str) -> Option<OsString>,
    has_database: bool,
) -> Result<Option<SpaceConfig>, ApplicationError> {
    const CATEGORY: &str = "FUNDGUARD_FINANCE_CATEGORY_ID";
    const ONBOARDING: &str = "FUNDGUARD_FINANCE_ONBOARDING_CHANNEL_IDS";
    let category = lookup(CATEGORY);
    let onboarding = lookup(ONBOARDING);
    if category.is_none() && onboarding.is_none() && !has_database {
        return Ok(None);
    }
    let category = category
        .map(|value| {
            let value = value
                .into_string()
                .map_err(|_| ApplicationError::configuration(CATEGORY))?;
            space_id(&value, CATEGORY)
        })
        .transpose()?;
    let mut channels = Vec::new();
    if let Some(value) = onboarding {
        let value = value
            .into_string()
            .map_err(|_| ApplicationError::configuration(ONBOARDING))?;
        for item in value.split(',') {
            let id = space_id(item.trim(), ONBOARDING)?;
            if channels.contains(&id) || channels.len() == 8 {
                return Err(ApplicationError::configuration(ONBOARDING));
            }
            channels.push(id);
        }
    }
    if !has_database {
        return Err(ApplicationError::configuration(
            "FUNDGUARD_FINANCE_DATABASE_URL",
        ));
    }
    Ok(Some(SpaceConfig {
        category,
        onboarding: channels,
    }))
}

fn space_id(value: &str, setting: &'static str) -> Result<NonZeroU64, ApplicationError> {
    if value.is_empty()
        || value.len() > 20
        || value.starts_with('0')
        || !value.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(ApplicationError::configuration(setting));
    }
    value
        .parse::<u64>()
        .ok()
        .and_then(NonZeroU64::new)
        .ok_or_else(|| ApplicationError::configuration(setting))
}

#[cfg(test)]
#[path = "discord_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "space_tests.rs"]
mod space_tests;
