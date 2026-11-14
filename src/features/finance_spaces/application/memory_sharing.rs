use super::*;
pub(super) fn prepare(
    store: &MemoryStore,
    input: SharingRequest,
) -> Result<SpaceOperation, SpaceError> {
    let mut spaces = store.spaces.lock().map_err(|_| SpaceError::Storage)?;
    let space = spaces
        .iter_mut()
        .find(|s| {
            s.id == input.space && s.revision == input.revision && s.generation == input.generation
        })
        .ok_or(SpaceError::Conflict)?;
    if space.current_operation.is_some() {
        return Err(SpaceError::Pending);
    }
    if !valid_source(&input.source)
        || input.kind == OperationKind::Create
        || input.target == space.member
    {
        return Err(SpaceError::Invalid);
    }
    if input.kind == OperationKind::GrantViewer && space.state != SpaceState::Ready {
        return Err(SpaceError::Permission);
    }
    let viewers = store.viewers.lock().map_err(|_| SpaceError::Storage)?;
    if (input.authority == SharingAuthority::Owner && space.member != input.actor)
        || (input.authority == SharingAuthority::InvalidatedGrant
            && (input.kind != OperationKind::RevokeViewer
                || input.actor == space.member
                || !input.source.starts_with("lifecycle:revoke:")
                || !viewers.iter().any(|v| {
                    v.space == space.id
                        && v.generation == space.generation
                        && v.member == input.target
                        && v.joined_at == input.joined_at
                        && v.desired == ViewerDesired::Revoked
                        && v.confirmed != ViewerConfirmed::Absent
                })))
    {
        return Err(SpaceError::Denied);
    }

    if input.kind == OperationKind::GrantViewer
        && viewers
            .iter()
            .filter(|v| {
                v.space == space.id
                    && v.generation == space.generation
                    && v.desired == ViewerDesired::Viewer
            })
            .count()
            >= 8
    {
        return Err(SpaceError::Capacity);
    }
    drop(viewers);
    space.revision = space.next_revision()?;
    let mut operations = store.operations.lock().map_err(|_| SpaceError::Storage)?;
    let op = SpaceOperation {
        id: format!(
            "{:08x}-1234-1234-1234-123456789aaa",
            operations.len() + 10000
        ),
        space: space.id,
        kind: input.kind,
        source: input.source,
        actor: input.actor,
        generation: space.generation.clone(),
        expected_revision: space.revision,
        target: Some(input.target),
        target_joined_at: Some(input.joined_at),
        phase: OperationPhase::Prepared,
        returned_channel: None,
        created_at: chrono::Utc::now(),
        dispatched_at: None,
        completed_at: None,
        outcome: None,
        desired_access: Some(if input.kind == OperationKind::GrantViewer {
            DesiredAccess::Viewer
        } else {
            DesiredAccess::Absent
        }),
    };
    space.current_operation = Some(op.id.clone());
    operations.push(op.clone());
    Ok(op)
}
pub(super) fn expire(store: &MemoryStore, id: SpaceId) -> Result<FinanceSpace, SpaceError> {
    finish(store, id, None, None)
}
pub(super) fn cancel(
    store: &MemoryStore,
    space: FinanceSpace,
    id: String,
) -> Result<FinanceSpace, SpaceError> {
    finish(store, space.id, Some(space.revision), Some(id))
}
fn finish(
    store: &MemoryStore,
    id: SpaceId,
    revision: Option<i64>,
    cancel: Option<String>,
) -> Result<FinanceSpace, SpaceError> {
    let mut spaces = store.spaces.lock().map_err(|_| SpaceError::Storage)?;
    let space = spaces
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or(SpaceError::Denied)?;
    if revision.is_some_and(|r| r != space.revision) {
        return Err(SpaceError::Conflict);
    }
    let Some(opid) = space.current_operation.clone() else {
        return Ok(space.clone());
    };
    let mut operations = store.operations.lock().map_err(|_| SpaceError::Storage)?;
    let op = operations
        .iter_mut()
        .find(|o| o.id == opid)
        .ok_or(SpaceError::Storage)?;
    if cancel.as_ref().is_some_and(|id| id != &op.id) {
        return Err(SpaceError::Denied);
    }
    if cancel.is_some() && (op.phase != OperationPhase::Prepared || op.dispatched_at.is_some()) {
        return Err(SpaceError::Pending);
    }
    if cancel.is_some() || op.prepared_expired(chrono::Utc::now()) {
        let cancelled = cancel.is_some();
        op.phase = OperationPhase::Failed;
        op.completed_at = Some(chrono::Utc::now());
        op.outcome = Some(if cancelled {
            OperationOutcome::Cancelled
        } else {
            OperationOutcome::Expired
        });
        space.revision = space.next_revision()?;
        space.current_operation = None;
        store
            .events
            .lock()
            .map_err(|_| SpaceError::Storage)?
            .push(AccessEvent {
                space: space.id,
                operation: Some(op.id.clone()),
                source: op.source.clone(),
                actor: op.actor,
                action: if cancelled {
                    AccessAction::Invalidated
                } else {
                    AccessAction::Expired
                },
                outcome: op.outcome.unwrap_or(OperationOutcome::Pending),
                at: chrono::Utc::now(),
            });
    }
    Ok(space.clone())
}
