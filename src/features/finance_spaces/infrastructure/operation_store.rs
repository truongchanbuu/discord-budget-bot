use super::super::{application::*, domain::*};
use super::{creation::audit, mapping, postgres::PgSpaceStore};

pub(super) async fn advance(
    store: &PgSpaceStore,
    space: FinanceSpace,
    op: SpaceOperation,
    step: OperationStep,
) -> Result<FinanceSpace, SpaceError> {
    if op.space != space.id
        || op.generation != space.generation
        || space.current_operation.as_ref() != Some(&op.id)
        || !op.phase.can_transition(step.next())
    {
        return Err(SpaceError::Conflict);
    }
    if matches!(step, OperationStep::ConfirmRemoval)
        && (op.kind != OperationKind::RevokeViewer
            || op.actor == space.member
            || !op.source.starts_with("lifecycle:revoke:"))
    {
        return Err(SpaceError::Denied);
    }
    space.next_revision()?;
    let (state, channel, action, outcome) = match step {
        OperationStep::Dispatch => (
            if op.kind == OperationKind::Create {
                SpaceState::Creating
            } else {
                SpaceState::AccessPending
            },
            space.channel,
            "dispatched",
            "pending",
        ),
        OperationStep::Candidate(channel) => {
            (SpaceState::Verifying, Some(channel), "verified", "pending")
        }
        OperationStep::Confirm => (
            SpaceState::Ready,
            space.channel,
            match op.kind {
                OperationKind::Create => "verified",
                OperationKind::GrantViewer => "granted",
                OperationKind::RevokeViewer => "revoked",
            },
            "confirmed",
        ),
        OperationStep::ConfirmRemoval => {
            (SpaceState::Suspended, space.channel, "revoked", "confirmed")
        }
        OperationStep::Uncertain => (
            SpaceState::Uncertain,
            space.channel,
            "uncertain",
            "uncertain",
        ),
    };
    let next = step.next();
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    if let OperationStep::Candidate(candidate) = step {
        let retired:bool=sqlx::query_scalar("SELECT EXISTS(SELECT FROM fg_space_operations WHERE returned_channel_id=$1 AND generation<>$2)").bind(candidate.get().to_string()).bind(&space.generation).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
        if retired {
            return Err(SpaceError::Conflict);
        }
    }
    let row=sqlx::query("UPDATE fg_finance_spaces SET state=$1,channel_id=$2,revision=revision+1,current_operation_id=CASE WHEN $3 THEN NULL ELSE current_operation_id END,updated_at=clock_timestamp() WHERE id=$4 AND revision=$5 AND generation=$6 AND current_operation_id=$7 RETURNING *")
        .bind(state.as_str()).bind(channel.map(|id|id.get().to_string())).bind(next.terminal()).bind(space.id.get()).bind(space.revision).bind(&space.generation).bind(&op.id).fetch_optional(&mut *tx).await.map_err(|_|SpaceError::Storage)?.ok_or(SpaceError::Conflict)?;
    let updated = mapping::space(row)?;
    let result=sqlx::query("UPDATE fg_space_operations SET phase=$1,returned_channel_id=COALESCE($2,returned_channel_id),dispatched_at=CASE WHEN $1='dispatched' THEN clock_timestamp() ELSE dispatched_at END,completed_at=CASE WHEN $3 THEN clock_timestamp() ELSE NULL END,outcome=CASE WHEN $3 OR $1='uncertain' THEN $4 ELSE NULL END WHERE id=$5 AND phase=$6 AND ($1<>'dispatched' OR kind='create' OR created_at>clock_timestamp()-INTERVAL '5 minutes')")
        .bind(next.as_str()).bind(channel.map(|id|id.get().to_string())).bind(next.terminal()).bind(outcome).bind(&op.id).bind(op.phase.as_str()).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    if result.rows_affected() != 1 {
        return Err(SpaceError::Conflict);
    }
    if op.kind != OperationKind::Create && next == OperationPhase::Dispatched {
        let target = op.target.ok_or(SpaceError::Invalid)?;
        let desired = if op.kind == OperationKind::GrantViewer {
            "viewer"
        } else {
            "revoked"
        };
        sqlx::query("INSERT INTO fg_space_viewers(space_id,generation,member_id,member_joined_at,desired_state,confirmed_state,operation_id,granted_by) VALUES($1,$2,$3,$4,$5,'pending',$6,$7) ON CONFLICT(space_id,generation,member_id) DO UPDATE SET member_joined_at=EXCLUDED.member_joined_at,desired_state=EXCLUDED.desired_state,confirmed_state='pending',operation_id=EXCLUDED.operation_id,revision=fg_space_viewers.revision+1,changed_at=clock_timestamp()")
            .bind(space.id.get()).bind(&space.generation).bind(target.get().to_string()).bind(super::incarnation::encode(op.target_joined_at.ok_or(SpaceError::Invalid)?)).bind(desired).bind(&op.id).bind(op.actor.get().to_string()).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    }
    if op.kind != OperationKind::Create && next == OperationPhase::Confirmed {
        let result=sqlx::query("UPDATE fg_space_viewers SET confirmed_state=$1,revision=revision+1,changed_at=clock_timestamp() WHERE space_id=$2 AND generation=$3 AND member_id=$4 AND operation_id=$5")
            .bind(if op.kind==OperationKind::GrantViewer {"viewer"} else {"absent"}).bind(space.id.get()).bind(&space.generation).bind(op.target.ok_or(SpaceError::Invalid)?.get().to_string()).bind(&op.id).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
        if result.rows_affected() != 1 {
            return Err(SpaceError::Conflict);
        }
    }
    let mut audited = updated.clone();
    audited.current_operation = Some(op.id);
    audit(&mut tx, &audited, &op.source, op.actor, action, outcome).await?;
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(updated)
}
