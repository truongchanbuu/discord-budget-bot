use super::*;
use crate::features::financial_core::OwnerRef;
use chrono::Utc;

pub(super) fn reserve(store: &MemoryStore, input: Reservation) -> Result<FinanceSpace, SpaceError> {
    input.validate()?;
    let mut spaces = store.spaces.lock().map_err(|_| SpaceError::Storage)?;
    if let Some(space) = spaces
        .iter()
        .find(|s| s.guild == input.guild && s.member == input.member)
    {
        return Ok(space.clone());
    }
    if spaces.len() >= 50 {
        return Err(SpaceError::Capacity);
    }
    let id = spaces.len() as i64 + 1;
    let generation = format!("{id:08x}-1234-1234-1234-123456789abc");
    let opid = format!("{id:08x}-1234-1234-1234-123456789def");
    let now = Utc::now();
    let legacy = input.legacy_owner.is_some();
    let journal_owner = match input.legacy_owner {
        Some(owner) => owner,
        None => OwnerRef::new(format!("owner-{id}")).map_err(|_| SpaceError::Invalid)?,
    };
    if spaces.iter().any(|s| s.journal_owner == journal_owner) {
        return Err(SpaceError::Conflict);
    }
    let space = FinanceSpace {
        id: SpaceId::new(id)?,
        guild: input.guild,
        member: input.member,
        journal_owner,
        legacy,
        owner_joined_at: Some(input.joined_at),
        generation: generation.clone(),
        channel: None,
        category: input.category,
        state: SpaceState::Prepared,
        revision: 0,
        current_operation: Some(opid.clone()),
    };
    store
        .operations
        .lock()
        .map_err(|_| SpaceError::Storage)?
        .push(SpaceOperation {
            id: opid,
            space: space.id,
            kind: OperationKind::Create,
            source: input.source,
            actor: input.member,
            generation,
            expected_revision: 0,
            target: None,
            target_joined_at: None,
            phase: OperationPhase::Prepared,
            returned_channel: None,
            created_at: now,
            dispatched_at: None,
            completed_at: None,
            outcome: None,
            desired_access: None,
        });
    spaces.push(space.clone());
    Ok(space)
}
pub(super) fn advance(
    store: &MemoryStore,
    expected: FinanceSpace,
    operation: SpaceOperation,
    step: OperationStep,
) -> Result<FinanceSpace, SpaceError> {
    if (matches!(step, OperationStep::Candidate(_))
        && store
            .fail_candidate_save
            .swap(false, std::sync::atomic::Ordering::Relaxed))
        || (matches!(step, OperationStep::Confirm)
            && store
                .fail_ready_save
                .swap(false, std::sync::atomic::Ordering::Relaxed))
    {
        return Err(SpaceError::Storage);
    }
    let mut spaces = store.spaces.lock().map_err(|_| SpaceError::Storage)?;
    let space = spaces
        .iter_mut()
        .find(|s| {
            s.id == expected.id
                && s.revision == expected.revision
                && s.generation == expected.generation
                && s.current_operation.as_ref() == Some(&operation.id)
        })
        .ok_or(SpaceError::Conflict)?;
    let mut operations = store.operations.lock().map_err(|_| SpaceError::Storage)?;
    let op = operations
        .iter_mut()
        .find(|o| o.id == operation.id && o.phase == operation.phase)
        .ok_or(SpaceError::Conflict)?;
    if !op.phase.can_transition(step.next()) {
        return Err(SpaceError::Conflict);
    }
    let now = Utc::now();
    let phase = step.next();
    if phase == OperationPhase::Dispatched && op.prepared_expired(now) {
        return Err(SpaceError::Conflict);
    }
    space.state = match step {
        OperationStep::Dispatch => {
            op.dispatched_at = Some(now);
            if op.kind == OperationKind::Create {
                SpaceState::Creating
            } else {
                SpaceState::AccessPending
            }
        }
        OperationStep::Candidate(channel) => {
            space.channel = Some(channel);
            op.returned_channel = Some(channel);
            SpaceState::Verifying
        }
        OperationStep::Confirm => {
            space.current_operation = None;
            op.completed_at = Some(now);
            op.outcome = Some(OperationOutcome::Confirmed);
            SpaceState::Ready
        }
        OperationStep::ConfirmRemoval => {
            space.current_operation = None;
            op.completed_at = Some(now);
            op.outcome = Some(OperationOutcome::Confirmed);
            SpaceState::Suspended
        }
        OperationStep::Uncertain => {
            op.outcome = Some(OperationOutcome::Uncertain);
            SpaceState::Uncertain
        }
    };
    if op.kind != OperationKind::Create {
        let mut viewers = store.viewers.lock().map_err(|_| SpaceError::Storage)?;
        if phase == OperationPhase::Dispatched {
            let target = op.target.ok_or(SpaceError::Invalid)?;
            viewers.retain(|v| {
                !(v.space == space.id && v.generation == space.generation && v.member == target)
            });
            viewers.push(ViewerGrant {
                space: space.id,
                generation: space.generation.clone(),
                member: target,
                joined_at: op.target_joined_at.ok_or(SpaceError::Invalid)?,
                desired: if op.kind == OperationKind::GrantViewer {
                    ViewerDesired::Viewer
                } else {
                    ViewerDesired::Revoked
                },
                confirmed: ViewerConfirmed::Pending,
                revision: 0,
                operation: op.id.clone(),
                granted_by: op.actor,
                changed_at: now,
            });
        } else if phase == OperationPhase::Confirmed {
            let viewer = viewers
                .iter_mut()
                .find(|v| v.operation == op.id)
                .ok_or(SpaceError::Conflict)?;
            viewer.confirmed = if op.kind == OperationKind::GrantViewer {
                ViewerConfirmed::Viewer
            } else {
                ViewerConfirmed::Absent
            };
        }
    }
    op.phase = phase;
    space.revision = space.next_revision()?;
    Ok(space.clone())
}
