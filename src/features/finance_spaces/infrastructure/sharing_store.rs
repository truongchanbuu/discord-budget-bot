use super::super::{application::*, domain::*};
use super::{creation::audit, mapping, postgres::PgSpaceStore};
use sqlx::{Postgres, Transaction};
async fn lock(tx: &mut Transaction<'_, Postgres>, id: SpaceId) -> Result<FinanceSpace, SpaceError> {
    mapping::space(
        sqlx::query("SELECT * FROM fg_finance_spaces WHERE id=$1 FOR UPDATE")
            .bind(id.get())
            .fetch_optional(&mut **tx)
            .await
            .map_err(|_| SpaceError::Storage)?
            .ok_or(SpaceError::Denied)?,
    )
}
async fn finish(
    tx: &mut Transaction<'_, Postgres>,
    mut space: FinanceSpace,
    op: &SpaceOperation,
    outcome: &str,
) -> Result<FinanceSpace, SpaceError> {
    sqlx::query("UPDATE fg_space_operations SET phase='failed',completed_at=clock_timestamp(),outcome=$1 WHERE id=$2 AND phase='prepared' AND dispatched_at IS NULL").bind(outcome).bind(&op.id).execute(&mut **tx).await.map_err(|_|SpaceError::Storage)?;
    space.revision = space.next_revision()?;
    sqlx::query("UPDATE fg_finance_spaces SET revision=$1,current_operation_id=NULL,updated_at=clock_timestamp() WHERE id=$2 AND current_operation_id=$3").bind(space.revision).bind(space.id.get()).bind(&op.id).execute(&mut **tx).await.map_err(|_|SpaceError::Storage)?;
    audit(
        tx,
        &space,
        &op.source,
        op.actor,
        if outcome == "expired" {
            "expired"
        } else {
            "invalidated"
        },
        outcome,
    )
    .await?;
    space.current_operation = None;
    Ok(space)
}
async fn expire_locked(
    tx: &mut Transaction<'_, Postgres>,
    space: FinanceSpace,
) -> Result<FinanceSpace, SpaceError> {
    let Some(id) = &space.current_operation else {
        return Ok(space);
    };
    let op = mapping::operation(
        sqlx::query("SELECT * FROM fg_space_operations WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_one(&mut **tx)
            .await
            .map_err(|_| SpaceError::Storage)?,
    )?;
    let now: chrono::DateTime<chrono::Utc> = sqlx::query_scalar("SELECT clock_timestamp()")
        .fetch_one(&mut **tx)
        .await
        .map_err(|_| SpaceError::Storage)?;
    if op.generation == space.generation && op.prepared_expired(now) {
        return finish(tx, space, &op, "expired").await;
    }
    Ok(space)
}
pub(super) async fn expire(store: &PgSpaceStore, id: SpaceId) -> Result<FinanceSpace, SpaceError> {
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    let space = lock(&mut tx, id).await?;
    let space = expire_locked(&mut tx, space).await?;
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(space)
}
pub(super) async fn cancel(
    store: &PgSpaceStore,
    expected: FinanceSpace,
    id: String,
) -> Result<FinanceSpace, SpaceError> {
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    let space = lock(&mut tx, expected.id).await?;
    if space.revision != expected.revision
        || space.generation != expected.generation
        || space.current_operation.as_ref() != Some(&id)
    {
        return Err(SpaceError::Conflict);
    }
    let op = mapping::operation(
        sqlx::query("SELECT * FROM fg_space_operations WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| SpaceError::Storage)?,
    )?;
    if op.phase != OperationPhase::Prepared || op.dispatched_at.is_some() {
        return Err(SpaceError::Pending);
    }
    let done = finish(&mut tx, space, &op, "cancelled").await?;
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(done)
}
pub(super) async fn prepare(
    store: &PgSpaceStore,
    input: SharingRequest,
) -> Result<SpaceOperation, SpaceError> {
    if !valid_source(&input.source)
        || input.kind == OperationKind::Create
        || input.actor == input.target
    {
        return Err(SpaceError::Invalid);
    }
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    let locked = lock(&mut tx, input.space).await?;
    let mut space = expire_locked(&mut tx, locked).await?;
    if (input.authority == SharingAuthority::Owner && space.member != input.actor)
        || space.revision != input.revision
        || space.generation != input.generation
    {
        return Err(SpaceError::Conflict);
    }
    if input.authority == SharingAuthority::InvalidatedGrant {
        let eligible:bool=sqlx::query_scalar("SELECT EXISTS(SELECT FROM fg_space_viewers WHERE space_id=$1 AND generation=$2 AND member_id=$3 AND member_joined_at=$4 AND desired_state='revoked' AND confirmed_state<>'absent')")
            .bind(space.id.get()).bind(&space.generation).bind(input.target.get().to_string()).bind(super::incarnation::encode(input.joined_at)).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
        if !eligible
            || input.actor == space.member
            || input.kind != OperationKind::RevokeViewer
            || !input.source.starts_with("lifecycle:revoke:")
        {
            return Err(SpaceError::Denied);
        }
    }
    if space.current_operation.is_some() || space.channel.is_none() {
        return Err(SpaceError::Pending);
    }
    if (input.kind == OperationKind::GrantViewer && space.state != SpaceState::Ready)
        || (input.kind == OperationKind::RevokeViewer
            && !matches!(
                space.state,
                SpaceState::Ready | SpaceState::Suspended | SpaceState::Verifying
            ))
    {
        return Err(SpaceError::Permission);
    }
    let changes:i64=sqlx::query_scalar("SELECT count(*) FROM fg_space_operations WHERE space_id=$1 AND kind<>'create' AND created_at>clock_timestamp()-INTERVAL '1 minute'").bind(space.id.get()).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    if changes >= 10 {
        return Err(SpaceError::RateLimited);
    }
    let viewers:i64=sqlx::query_scalar("SELECT count(*) FROM fg_space_viewers WHERE space_id=$1 AND generation=$2 AND desired_state='viewer'").bind(space.id.get()).bind(&space.generation).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT FROM fg_space_viewers WHERE space_id=$1 AND generation=$2 AND member_id=$3)").bind(space.id.get()).bind(&space.generation).bind(input.target.get().to_string()).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    if input.kind == OperationKind::GrantViewer && viewers >= 8 {
        return Err(SpaceError::Capacity);
    }
    if input.kind == OperationKind::RevokeViewer && !exists {
        return Err(SpaceError::Denied);
    }
    space.revision = space.next_revision()?;
    let desired = if input.kind == OperationKind::GrantViewer {
        "viewer"
    } else {
        "absent"
    };
    let op=mapping::operation(sqlx::query("INSERT INTO fg_space_operations(space_id,kind,source,actor,generation,expected_revision,target_member,target_joined_at,desired_access,phase) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,'prepared') RETURNING *")
        .bind(space.id.get()).bind(input.kind.as_str()).bind(&input.source).bind(input.actor.get().to_string()).bind(&space.generation).bind(space.revision).bind(input.target.get().to_string()).bind(super::incarnation::encode(input.joined_at)).bind(desired).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Conflict)?)?;
    space.current_operation = Some(op.id.clone());
    sqlx::query("UPDATE fg_finance_spaces SET revision=$1,current_operation_id=$2,updated_at=clock_timestamp() WHERE id=$3").bind(space.revision).bind(&op.id).bind(space.id.get()).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    audit(&mut tx, &space, &op.source, op.actor, "reserved", "pending").await?;
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(op)
}
