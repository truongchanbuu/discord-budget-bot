use super::{MemberRef, OperationOutcome, SpaceError, SpaceId, valid_source, valid_token};
use chrono::{DateTime, Utc};
use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ViewerDesired {
    Viewer,
    Revoked,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ViewerConfirmed {
    Pending,
    Viewer,
    Absent,
    Uncertain,
}
#[derive(Clone)]
pub(crate) struct ViewerGrant {
    pub(crate) space: SpaceId,
    pub(crate) generation: String,
    pub(crate) member: MemberRef,
    pub(crate) joined_at: DateTime<Utc>,
    pub(crate) desired: ViewerDesired,
    pub(crate) confirmed: ViewerConfirmed,
    pub(crate) revision: i64,
    pub(crate) operation: String,
    pub(crate) granted_by: MemberRef,
    pub(crate) changed_at: DateTime<Utc>,
}
impl fmt::Debug for ViewerGrant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ViewerGrant([redacted])")
    }
}
impl ViewerGrant {
    pub(crate) fn validate(&self, owner: MemberRef, bot: MemberRef) -> Result<(), SpaceError> {
        if self.member == owner
            || self.member == bot
            || self.granted_by != owner
            || self.revision < 0
            || !valid_token(&self.generation)
            || !valid_token(&self.operation)
        {
            return Err(SpaceError::Invalid);
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessAction {
    Reserved,
    Verified,
    Granted,
    Revoked,
    Invalidated,
    Retired,
    Recovered,
    Expired,
    Dispatched,
    Uncertain,
}
impl AccessAction {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Reserved => "reserved",
            Self::Verified => "verified",
            Self::Granted => "granted",
            Self::Revoked => "revoked",
            Self::Invalidated => "invalidated",
            Self::Retired => "retired",
            Self::Recovered => "recovered",
            Self::Expired => "expired",
            Self::Dispatched => "dispatched",
            Self::Uncertain => "uncertain",
        }
    }
}
pub(crate) struct AccessEvent {
    pub(crate) space: SpaceId,
    pub(crate) operation: Option<String>,
    pub(crate) source: String,
    pub(crate) actor: MemberRef,
    pub(crate) action: AccessAction,
    pub(crate) outcome: OperationOutcome,
    pub(crate) at: DateTime<Utc>,
}
impl AccessEvent {
    pub(crate) fn validate(&self) -> Result<(), SpaceError> {
        if !valid_source(&self.source) || self.operation.as_ref().is_some_and(|id| !valid_token(id))
        {
            return Err(SpaceError::Invalid);
        }
        Ok(())
    }
}
impl fmt::Debug for AccessEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AccessEvent([redacted])")
    }
}

impl AccessAction {
    pub(crate) fn parse(value: &str) -> Result<Self, SpaceError> {
        match value {
            "reserved" => Ok(Self::Reserved),
            "verified" => Ok(Self::Verified),
            "granted" => Ok(Self::Granted),
            "revoked" => Ok(Self::Revoked),
            "invalidated" => Ok(Self::Invalidated),
            "retired" => Ok(Self::Retired),
            "recovered" => Ok(Self::Recovered),
            "expired" => Ok(Self::Expired),
            "dispatched" => Ok(Self::Dispatched),
            "uncertain" => Ok(Self::Uncertain),
            _ => Err(SpaceError::Invalid),
        }
    }
}
