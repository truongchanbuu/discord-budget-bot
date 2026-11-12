use std::{fmt, num::NonZeroU64};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum AccessError {
    #[error("invalid account identifier")]
    InvalidAccountId,
    #[error("access denied")]
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingState {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DiscordAccountId(NonZeroU64);

impl DiscordAccountId {
    pub fn parse(input: &str) -> Result<Self, AccessError> {
        if input.is_empty()
            || input.len() > 20
            || input.starts_with('0')
            || !input.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(AccessError::InvalidAccountId);
        }
        input
            .parse::<u64>()
            .ok()
            .and_then(NonZeroU64::new)
            .map(Self)
            .ok_or(AccessError::InvalidAccountId)
    }
}

impl fmt::Debug for DiscordAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DiscordAccountId([redacted])")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TrustedActor(DiscordAccountId);

impl TrustedActor {
    pub fn new(account: DiscordAccountId) -> Self {
        Self(account)
    }

    pub(crate) fn account(&self) -> DiscordAccountId {
        self.0
    }
}

impl fmt::Debug for TrustedActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TrustedActor([redacted])")
    }
}
