use super::{ChannelRef, MemberRef, SpaceError, SpaceId, valid_token};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperationKind {
    Create,
    GrantViewer,
    RevokeViewer,
}
impl OperationKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::GrantViewer => "grant_viewer",
            Self::RevokeViewer => "revoke_viewer",
        }
    }
    pub(crate) fn parse(value: &str) -> Result<Self, SpaceError> {
        match value {
            "create" => Ok(Self::Create),
            "grant_viewer" => Ok(Self::GrantViewer),
            "revoke_viewer" => Ok(Self::RevokeViewer),
            _ => Err(SpaceError::Invalid),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperationPhase {
    Prepared,
    Dispatched,
    Uncertain,
    Verifying,
    Confirmed,
    Failed,
    Retired,
}
impl OperationPhase {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Dispatched => "dispatched",
            Self::Uncertain => "uncertain",
            Self::Verifying => "verifying",
            Self::Confirmed => "confirmed",
            Self::Failed => "failed",
            Self::Retired => "retired",
        }
    }
    pub(crate) fn parse(value: &str) -> Result<Self, SpaceError> {
        match value {
            "prepared" => Ok(Self::Prepared),
            "dispatched" => Ok(Self::Dispatched),
            "uncertain" => Ok(Self::Uncertain),
            "verifying" => Ok(Self::Verifying),
            "confirmed" => Ok(Self::Confirmed),
            "failed" => Ok(Self::Failed),
            "retired" => Ok(Self::Retired),
            _ => Err(SpaceError::Invalid),
        }
    }
    pub(crate) fn terminal(self) -> bool {
        matches!(self, Self::Confirmed | Self::Failed | Self::Retired)
    }
    pub(crate) fn can_transition(self, next: Self) -> bool {
        !self.terminal()
            && (next == Self::Retired
                || matches!(
                    (self, next),
                    (Self::Prepared, Self::Dispatched | Self::Failed)
                        | (Self::Dispatched, Self::Uncertain | Self::Verifying)
                        | (Self::Uncertain, Self::Verifying)
                        | (Self::Verifying, Self::Confirmed | Self::Uncertain)
                ))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesiredAccess {
    Viewer,
    Absent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OperationOutcome {
    Pending,
    Confirmed,
    Cancelled,
    Expired,
    Denied,
    Permission,
    Timeout,
    Uncertain,
    Retired,
}
#[derive(Clone)]
pub(crate) struct SpaceOperation {
    pub(crate) id: String,
    pub(crate) space: SpaceId,
    pub(crate) kind: OperationKind,
    pub(crate) source: String,
    pub(crate) actor: MemberRef,
    pub(crate) generation: String,
    pub(crate) expected_revision: i64,
    pub(crate) target: Option<MemberRef>,
    pub(crate) target_joined_at: Option<DateTime<Utc>>,
    pub(crate) phase: OperationPhase,
    pub(crate) returned_channel: Option<ChannelRef>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) dispatched_at: Option<DateTime<Utc>>,
    pub(crate) completed_at: Option<DateTime<Utc>>,
    pub(crate) outcome: Option<OperationOutcome>,
    pub(crate) desired_access: Option<DesiredAccess>,
}
impl fmt::Debug for SpaceOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SpaceOperation([redacted])")
    }
}
impl SpaceOperation {
    pub(crate) fn validate(&self) -> Result<(), SpaceError> {
        let sharing = self.kind != OperationKind::Create;
        if self.phase.terminal() != self.completed_at.is_some()
            || (self.phase.terminal() && self.outcome.is_none())
            || (self.phase == OperationPhase::Confirmed
                && self.outcome != Some(OperationOutcome::Confirmed))
            || (matches!(
                self.phase,
                OperationPhase::Verifying | OperationPhase::Confirmed
            ) && self.returned_channel.is_none())
        {
            return Err(SpaceError::Invalid);
        }
        if !valid_token(&self.id)
            || !valid_token(&self.generation)
            || !valid_source(&self.source)
            || self.expected_revision < 0
            || sharing != self.target.is_some()
            || sharing != self.target_joined_at.is_some()
            || self.target == Some(self.actor)
            || self.desired_access
                != match self.kind {
                    OperationKind::Create => None,
                    OperationKind::GrantViewer => Some(DesiredAccess::Viewer),
                    OperationKind::RevokeViewer => Some(DesiredAccess::Absent),
                }
        {
            return Err(SpaceError::Invalid);
        }
        Ok(())
    }
    pub(crate) fn prepared_expired(&self, now: DateTime<Utc>) -> bool {
        self.kind != OperationKind::Create
            && self.phase == OperationPhase::Prepared
            && self.dispatched_at.is_none()
            && now.signed_duration_since(self.created_at) >= chrono::Duration::minutes(5)
    }
}
pub(crate) fn valid_source(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128 && value.bytes().all(|b| b.is_ascii_graphic())
}

impl OperationOutcome {
    pub(crate) fn parse(value: &str) -> Result<Self, SpaceError> {
        match value {
            "pending" => Ok(Self::Pending),
            "confirmed" => Ok(Self::Confirmed),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            "denied" => Ok(Self::Denied),
            "permission" => Ok(Self::Permission),
            "timeout" => Ok(Self::Timeout),
            "uncertain" => Ok(Self::Uncertain),
            "retired" => Ok(Self::Retired),
            _ => Err(SpaceError::Invalid),
        }
    }
}

impl OperationOutcome {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Confirmed => "confirmed",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
            Self::Denied => "denied",
            Self::Permission => "permission",
            Self::Timeout => "timeout",
            Self::Uncertain => "uncertain",
            Self::Retired => "retired",
        }
    }
}
