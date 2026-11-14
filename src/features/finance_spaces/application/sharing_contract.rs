use super::super::domain::*;
use chrono::{DateTime, Utc};
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum SharingAuthority {
    Owner,
    InvalidatedGrant,
}
pub(crate) struct SharingRequest {
    pub(crate) authority: SharingAuthority,
    pub(crate) space: SpaceId,
    pub(crate) revision: i64,
    pub(crate) generation: String,
    pub(crate) actor: MemberRef,
    pub(crate) target: MemberRef,
    pub(crate) joined_at: DateTime<Utc>,
    pub(crate) kind: OperationKind,
    pub(crate) source: String,
}
pub(crate) struct ViewerRequest {
    pub(crate) member: MemberRef,
    pub(crate) channel: ChannelRef,
    pub(crate) space: SpaceId,
    pub(crate) target: MemberRef,
    pub(crate) kind: OperationKind,
    pub(crate) source: String,
}
