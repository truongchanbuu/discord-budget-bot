use super::*;
pub(super) fn invalidate(
    store: &MemoryStore,
    guild: ServerRef,
    event: LifecycleEvent,
) -> Result<(), SpaceError> {
    let mut spaces = store.spaces.lock().map_err(|_| SpaceError::Storage)?;
    let mut ops = store.operations.lock().map_err(|_| SpaceError::Storage)?;
    let mut viewers = store.viewers.lock().map_err(|_| SpaceError::Storage)?;
    for space in spaces.iter_mut().filter(|s| s.guild == guild) {
        let affected = match event {
            LifecycleEvent::VerifyAll => true,
            LifecycleEvent::ChannelChanged(id) | LifecycleEvent::ChannelDeleted(id) => {
                space.channel == Some(id) || space.category == id
            }
            LifecycleEvent::MemberRemoved(id) | LifecycleEvent::MemberChanged(id) => {
                space.member == id
                    || viewers.iter().any(|v| {
                        v.space == space.id
                            && v.generation == space.generation
                            && v.member == id
                            && v.confirmed != ViewerConfirmed::Absent
                    })
            }
        };
        if !affected {
            continue;
        }
        if let LifecycleEvent::MemberRemoved(id) | LifecycleEvent::MemberChanged(id) = event {
            for viewer in viewers.iter_mut().filter(|v| {
                v.space == space.id
                    && v.generation == space.generation
                    && v.member == id
                    && v.confirmed != ViewerConfirmed::Absent
            }) {
                viewer.desired = ViewerDesired::Revoked;
                viewer.confirmed = ViewerConfirmed::Uncertain;
                viewer.revision += 1;
            }
        }
        let retired = matches!(event,LifecycleEvent::MemberRemoved(id) if id==space.member)
            || matches!(event,LifecycleEvent::ChannelDeleted(id) if Some(id)==space.channel);
        if retired {
            space.state = SpaceState::Retired;
            space.current_operation = None;
            for op in ops
                .iter_mut()
                .filter(|o| o.space == space.id && !o.phase.terminal())
            {
                op.phase = OperationPhase::Retired;
                op.completed_at = Some(chrono::Utc::now());
                op.outcome = Some(OperationOutcome::Retired);
            }
        } else if space.state == SpaceState::Ready {
            space.state = SpaceState::Verifying;
        }
        space.revision = space.next_revision()?;
    }
    Ok(())
}
pub(super) fn recover(
    store: &MemoryStore,
    input: RecoveryRequest,
) -> Result<FinanceSpace, SpaceError> {
    let mut spaces = store.spaces.lock().map_err(|_| SpaceError::Storage)?;
    let space = spaces
        .iter_mut()
        .find(|s| {
            s.id == input.space
                && s.member == input.member
                && s.revision == input.expected_revision
                && s.state == SpaceState::Retired
        })
        .ok_or(SpaceError::Denied)?;
    if space.current_operation.is_some() {
        return Err(SpaceError::Pending);
    }
    space.revision = space.next_revision()?;
    space.owner_joined_at = Some(input.joined_at);
    space.channel = None;
    space.category = input.category;
    space.generation = format!("{:08x}-1234-1234-1234-123456789abc", space.revision + 1000);
    let opid = format!("{:08x}-1234-1234-1234-123456789def", space.revision + 1000);
    space.current_operation = Some(opid.clone());
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
            generation: space.generation.clone(),
            expected_revision: space.revision,
            target: None,
            target_joined_at: None,
            phase: OperationPhase::Prepared,
            returned_channel: None,
            created_at: chrono::Utc::now(),
            dispatched_at: None,
            completed_at: None,
            outcome: None,
            desired_access: None,
        });
    Ok(space.clone())
}
