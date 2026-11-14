use super::super::domain::*;
use super::*;
use std::sync::Mutex;
#[derive(Default)]
pub(super) struct MemoryStore {
    pub(super) spaces: Mutex<Vec<FinanceSpace>>,
    pub(super) events: Mutex<Vec<AccessEvent>>,
    pub(super) operations: Mutex<Vec<SpaceOperation>>,
    pub(super) fail_candidate_save: std::sync::atomic::AtomicBool,
    pub(super) fail_ready_save: std::sync::atomic::AtomicBool,
    pub(super) viewers: Mutex<Vec<ViewerGrant>>,
}
impl SpaceStore for MemoryStore {
    fn viewers(&self, space: FinanceSpace) -> SpaceFuture<'_, Vec<ViewerGrant>> {
        Box::pin(async move {
            Ok(self
                .viewers
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .filter(|v| v.space == space.id && v.generation == space.generation)
                .cloned()
                .collect())
        })
    }
    fn prepare_share(&self, input: SharingRequest) -> SpaceFuture<'_, SpaceOperation> {
        Box::pin(async move { sharing::prepare(self, input) })
    }
    fn expire_prepared(&self, id: SpaceId) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move { sharing::expire(self, id) })
    }
    fn cancel_prepared(&self, space: FinanceSpace, id: String) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move { sharing::cancel(self, space, id) })
    }

    fn invalidate(&self, guild: ServerRef, event: LifecycleEvent) -> SpaceFuture<'_, ()> {
        Box::pin(async move { lifecycle::invalidate(self, guild, event) })
    }
    fn tracked(&self, guild: ServerRef, after: i64) -> SpaceFuture<'_, Vec<FinanceSpace>> {
        Box::pin(async move {
            Ok(self
                .spaces
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .filter(|s| s.guild == guild && s.id.get() > after)
                .take(2)
                .cloned()
                .collect())
        })
    }
    fn recover(&self, input: RecoveryRequest) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move { lifecycle::recover(self, input) })
    }
    fn check_legacy(&self, input: LegacyCheck) -> SpaceFuture<'_, ()> {
        Box::pin(async move {
            let Some(owner) = input.owner else {
                return Ok(());
            };
            if self
                .spaces
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .any(|s| {
                    (s.guild == input.guild
                        && s.legacy
                        && (s.member != input.member || s.journal_owner != owner))
                        || (s.journal_owner == owner
                            && (s.guild != input.guild || s.member != input.member))
                })
            {
                return Err(SpaceError::Conflict);
            }
            Ok(())
        })
    }

    fn reserve(&self, input: Reservation) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move { creation::reserve(self, input) })
    }
    fn operation(&self, id: String) -> SpaceFuture<'_, SpaceOperation> {
        Box::pin(async move {
            self.operations
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .find(|o| o.id == id)
                .cloned()
                .ok_or(SpaceError::Denied)
        })
    }
    fn advance(
        &self,
        space: FinanceSpace,
        operation: SpaceOperation,
        step: OperationStep,
    ) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move { creation::advance(self, space, operation, step) })
    }

    fn find(&self, guild: ServerRef, member: MemberRef) -> SpaceFuture<'_, Option<FinanceSpace>> {
        Box::pin(async move {
            Ok(self
                .spaces
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .find(|s| s.guild == guild && s.member == member)
                .cloned())
        })
    }
    fn load(&self, id: SpaceId) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move {
            self.spaces
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .find(|s| s.id == id)
                .cloned()
                .ok_or(SpaceError::Denied)
        })
    }
    fn transition(&self, change: SpaceTransition) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move {
            change.validate()?;
            let mut spaces = self.spaces.lock().map_err(|_| SpaceError::Storage)?;
            let space = spaces
                .iter_mut()
                .find(|s| {
                    s.id == change.space
                        && s.revision == change.expected_revision
                        && s.generation == change.generation
                })
                .ok_or(SpaceError::Conflict)?;
            let mut events = self.events.lock().map_err(|_| SpaceError::Storage)?;
            space.revision = space.next_revision()?;
            space.state = change.state;
            events.push(AccessEvent {
                space: space.id,
                operation: space.current_operation.clone(),
                source: change.source,
                actor: change.actor,
                action: change.action,
                outcome: OperationOutcome::Pending,
                at: chrono::Utc::now(),
            });
            Ok(space.clone())
        })
    }
}

#[derive(Default)]
pub(super) struct MemoryDirectory {
    pub(super) members: Mutex<Vec<(ServerRef, MemberSnapshot)>>,
    pub(super) channels: Mutex<Vec<ChannelSnapshot>>,
    pub(super) creates: std::sync::atomic::AtomicU64,
    pub(super) fail_create: std::sync::atomic::AtomicBool,
    pub(super) fail_inspect: std::sync::atomic::AtomicBool,
    pub(super) missing: Mutex<Vec<ChannelRef>>,
    pub(super) fail_change: std::sync::atomic::AtomicBool,
    pub(super) changes: std::sync::atomic::AtomicU64,
}
#[tokio::test]
async fn memory_ports_preserve_identity_and_reject_missing_observations() {
    let store = MemoryStore::default();
    assert!(
        store
            .find(
                ServerRef::parse("10").unwrap(),
                MemberRef::parse("11").unwrap()
            )
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(
        store.load(SpaceId::new(1).unwrap()).await.unwrap_err(),
        SpaceError::Denied
    );
    let directory = MemoryDirectory::default();
    assert_eq!(
        directory
            .inspect(
                ChannelRef::parse("12").unwrap(),
                MemberRef::parse("99").unwrap()
            )
            .await
            .unwrap_err(),
        SpaceError::Permission
    );
}

#[path = "memory_creation.rs"]
mod creation;
#[path = "memory_directory.rs"]
mod directory;

#[path = "memory_lifecycle.rs"]
mod lifecycle;

#[path = "memory_sharing.rs"]
mod sharing;

#[path = "memory_observations.rs"]
mod observations;
