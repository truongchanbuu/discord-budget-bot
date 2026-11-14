use super::super::domain::*;
use crate::features::financial_core::OwnerRef;
use chrono::{DateTime, Utc};
#[derive(Clone, Copy)]
pub(crate) enum LifecycleEvent {
    VerifyAll,
    ChannelChanged(ChannelRef),
    ChannelDeleted(ChannelRef),
    MemberRemoved(MemberRef),
    MemberChanged(MemberRef),
}
pub(crate) struct RecoveryRequest {
    pub(crate) space: SpaceId,
    pub(crate) expected_revision: i64,
    pub(crate) category: ChannelRef,
    pub(crate) member: MemberRef,
    pub(crate) joined_at: DateTime<Utc>,
    pub(crate) source: String,
}
pub(crate) struct LegacyCheck {
    pub(crate) guild: ServerRef,
    pub(crate) member: MemberRef,
    pub(crate) owner: Option<OwnerRef>,
}
