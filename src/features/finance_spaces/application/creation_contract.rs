use super::super::domain::*;
use crate::features::financial_core::OwnerRef;
use chrono::{DateTime, Utc};

pub(crate) struct Reservation {
    pub(crate) guild: ServerRef,
    pub(crate) member: MemberRef,
    pub(crate) category: ChannelRef,
    pub(crate) joined_at: DateTime<Utc>,
    pub(crate) legacy_owner: Option<OwnerRef>,
    pub(crate) source: String,
}
impl Reservation {
    pub(crate) fn validate(&self) -> Result<(), SpaceError> {
        if !valid_source(&self.source) {
            return Err(SpaceError::Invalid);
        }
        Ok(())
    }
}
pub(crate) enum OperationStep {
    Dispatch,
    Candidate(ChannelRef),
    Confirm,
    ConfirmRemoval,
    Uncertain,
}
impl OperationStep {
    pub(crate) fn next(&self) -> OperationPhase {
        match self {
            Self::Dispatch => OperationPhase::Dispatched,
            Self::Candidate(_) => OperationPhase::Verifying,
            Self::Confirm | Self::ConfirmRemoval => OperationPhase::Confirmed,
            Self::Uncertain => OperationPhase::Uncertain,
        }
    }
}

pub(crate) struct ChannelCreation {
    pub(crate) guild: ServerRef,
    pub(crate) category: ChannelRef,
    pub(crate) owner: MemberRef,
    pub(crate) bot: MemberRef,
    pub(crate) generation: String,
}
