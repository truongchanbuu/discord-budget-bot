use super::super::domain::*;
use super::{
    ChannelCreation, LegacyCheck, LifecycleEvent, OperationStep, RecoveryRequest, Reservation,
    SharingRequest,
};
use std::{future::Future, pin::Pin};

pub(crate) type SpaceFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, SpaceError>> + Send + 'a>>;
pub(crate) struct SpaceTransition {
    pub(crate) space: SpaceId,
    pub(crate) expected_revision: i64,
    pub(crate) generation: String,
    pub(crate) state: SpaceState,
    pub(crate) source: String,
    pub(crate) actor: MemberRef,
    pub(crate) action: AccessAction,
}
impl SpaceTransition {
    pub(crate) fn validate(&self) -> Result<(), SpaceError> {
        if self.expected_revision < 0
            || self.expected_revision == i64::MAX
            || !valid_token(&self.generation)
            || !valid_source(&self.source)
        {
            return Err(SpaceError::Invalid);
        }
        Ok(())
    }
}

pub(crate) trait SpaceStore: Send + Sync {
    fn viewers(&self, space: FinanceSpace) -> SpaceFuture<'_, Vec<ViewerGrant>>;
    fn prepare_share(&self, input: SharingRequest) -> SpaceFuture<'_, SpaceOperation>;
    fn expire_prepared(&self, id: SpaceId) -> SpaceFuture<'_, FinanceSpace>;
    fn cancel_prepared(&self, space: FinanceSpace, id: String) -> SpaceFuture<'_, FinanceSpace>;
    fn invalidate(&self, guild: ServerRef, event: LifecycleEvent) -> SpaceFuture<'_, ()>;
    fn tracked(&self, guild: ServerRef, after: i64) -> SpaceFuture<'_, Vec<FinanceSpace>>;
    fn recover(&self, input: RecoveryRequest) -> SpaceFuture<'_, FinanceSpace>;
    fn check_legacy(&self, input: LegacyCheck) -> SpaceFuture<'_, ()>;
    fn reserve(&self, input: Reservation) -> SpaceFuture<'_, FinanceSpace>;
    fn operation(&self, id: String) -> SpaceFuture<'_, SpaceOperation>;
    fn advance(
        &self,
        space: FinanceSpace,
        operation: SpaceOperation,
        step: OperationStep,
    ) -> SpaceFuture<'_, FinanceSpace>;
    fn find(&self, guild: ServerRef, member: MemberRef) -> SpaceFuture<'_, Option<FinanceSpace>>;
    fn load(&self, id: SpaceId) -> SpaceFuture<'_, FinanceSpace>;
    fn transition(&self, change: SpaceTransition) -> SpaceFuture<'_, FinanceSpace>;
}

pub(crate) trait ChannelDirectory: Send + Sync {
    fn create(&self, input: ChannelCreation) -> SpaceFuture<'_, ChannelRef>;
    fn change_viewer(
        &self,
        channel: ChannelRef,
        member: MemberRef,
        access: DesiredAccess,
    ) -> SpaceFuture<'_, ()>;
    fn candidates(
        &self,
        guild: ServerRef,
        category: ChannelRef,
        generation: String,
    ) -> SpaceFuture<'_, Vec<ChannelRef>>;
    fn setup(
        &self,
        guild: ServerRef,
        category: ChannelRef,
        onboarding: Vec<ChannelRef>,
        bot: MemberRef,
    ) -> SpaceFuture<'_, ()>;
    fn member(&self, guild: ServerRef, member: MemberRef) -> SpaceFuture<'_, MemberSnapshot>;
    fn inspect(&self, channel: ChannelRef, bot: MemberRef) -> SpaceFuture<'_, ChannelSnapshot>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transitions_reject_unbounded_sources_and_invalid_revisions() {
        let mut change = SpaceTransition {
            space: SpaceId::new(1).unwrap(),
            expected_revision: 0,
            generation: "12345678-1234-1234-1234-123456789abc".into(),
            state: SpaceState::Suspended,
            source: "discord:guild:100".into(),
            actor: MemberRef::parse("11").unwrap(),
            action: AccessAction::Invalidated,
        };
        assert!(change.validate().is_ok());
        change.expected_revision = i64::MAX;
        assert!(change.validate().is_err());
        change.expected_revision = 0;
        change.source = "secret\nmarker".into();
        assert!(change.validate().is_err());
    }
}
