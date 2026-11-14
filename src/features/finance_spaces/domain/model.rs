use super::SpaceError;
use crate::features::{financial_core::OwnerRef, identity::DiscordAccountId};
use chrono::{DateTime, Utc};
use std::fmt;

macro_rules! reference {
    ($name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub(crate) struct $name(u64);
        impl $name {
            pub(crate) fn parse(value: &str) -> Result<Self, SpaceError> {
                DiscordAccountId::parse(value)
                    .map(|id| Self(id.as_u64()))
                    .map_err(|_| SpaceError::Invalid)
            }
            pub(crate) fn get(self) -> u64 {
                self.0
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), "([redacted])"))
            }
        }
    };
}
reference!(ServerRef);
reference!(ChannelRef);
reference!(MemberRef);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct SpaceId(i64);
impl SpaceId {
    pub(crate) fn new(value: i64) -> Result<Self, SpaceError> {
        if value <= 0 {
            return Err(SpaceError::Invalid);
        }
        Ok(Self(value))
    }
    pub(crate) fn get(self) -> i64 {
        self.0
    }
}
impl fmt::Debug for SpaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SpaceId([redacted])")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpaceState {
    Prepared,
    Creating,
    Uncertain,
    Verifying,
    Ready,
    AccessPending,
    Suspended,
    Retired,
}
impl SpaceState {
    pub(crate) fn parse(value: &str) -> Result<Self, SpaceError> {
        match value {
            "prepared" => Ok(Self::Prepared),
            "creating" => Ok(Self::Creating),
            "uncertain" => Ok(Self::Uncertain),
            "verifying" => Ok(Self::Verifying),
            "ready" => Ok(Self::Ready),
            "access_pending" => Ok(Self::AccessPending),
            "suspended" => Ok(Self::Suspended),
            "retired" => Ok(Self::Retired),
            _ => Err(SpaceError::Invalid),
        }
    }
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Creating => "creating",
            Self::Uncertain => "uncertain",
            Self::Verifying => "verifying",
            Self::Ready => "ready",
            Self::AccessPending => "access_pending",
            Self::Suspended => "suspended",
            Self::Retired => "retired",
        }
    }
    pub(crate) fn permits_finance(self) -> bool {
        self == Self::Ready
    }
}

#[derive(Clone)]
pub(crate) struct FinanceSpace {
    pub(crate) id: SpaceId,
    pub(crate) guild: ServerRef,
    pub(crate) member: MemberRef,
    pub(crate) journal_owner: OwnerRef,
    pub(crate) legacy: bool,
    pub(crate) owner_joined_at: Option<DateTime<Utc>>,
    pub(crate) generation: String,
    pub(crate) channel: Option<ChannelRef>,
    pub(crate) category: ChannelRef,
    pub(crate) state: SpaceState,
    pub(crate) revision: i64,
    pub(crate) current_operation: Option<String>,
}
impl fmt::Debug for FinanceSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FinanceSpace([redacted])")
    }
}
impl FinanceSpace {
    pub(crate) fn is_bound_ready(&self) -> bool {
        self.state.permits_finance()
            && self.channel.is_some()
            && self.owner_joined_at.is_some()
            && valid_token(&self.generation)
            && self.revision >= 0
    }
    pub(crate) fn next_revision(&self) -> Result<i64, SpaceError> {
        self.revision.checked_add(1).ok_or(SpaceError::Conflict)
    }
}

pub(crate) fn valid_token(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit() && !b.is_ascii_uppercase()
            }
        })
}
