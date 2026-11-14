use super::super::{application::*, domain::*};
use super::{
    creation::{audit, guild_lock, quota},
    mapping,
    postgres::PgSpaceStore,
};
pub(super) async fn prepare(
    store: &PgSpaceStore,
    input: RecoveryRequest,
) -> Result<FinanceSpace, SpaceError> {
    if !valid_source(&input.source) {
        return Err(SpaceError::Invalid);
    }
    let initial = store.load(input.space).await?;
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    guild_lock(&mut tx, initial.guild).await?;
    let mut space = mapping::space(
        sqlx::query("SELECT * FROM fg_finance_spaces WHERE id=$1 FOR UPDATE")
            .bind(input.space.get())
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| SpaceError::Storage)?,
    )?;
    if space.member != input.member
        || space.state != SpaceState::Retired
        || space.revision != input.expected_revision
    {
        return Err(SpaceError::Denied);
    }
    if space.current_operation.is_some() {
        return Err(SpaceError::Pending);
    }
    quota(&mut tx, space.guild, space.member).await?;
    space.revision = space.next_revision()?;
    let row=sqlx::query("UPDATE fg_finance_spaces SET generation=gen_random_uuid()::text,channel_id=NULL,owner_joined_at=$1,revision=$2,category_id=$4,updated_at=clock_timestamp() WHERE id=$3 RETURNING *").bind(super::incarnation::encode(input.joined_at)).bind(space.revision).bind(space.id.get()).bind(input.category.get().to_string()).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    space = mapping::space(row)?;
    let operation:String=sqlx::query_scalar("INSERT INTO fg_space_operations(space_id,kind,source,actor,generation,expected_revision,phase) VALUES($1,'create',$2,$3,$4,$5,'prepared') RETURNING id")
        .bind(space.id.get()).bind(&input.source).bind(space.member.get().to_string()).bind(&space.generation).bind(space.revision).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Conflict)?;
    space.current_operation = Some(operation);
    sqlx::query("UPDATE fg_finance_spaces SET current_operation_id=$1 WHERE id=$2")
        .bind(&space.current_operation)
        .bind(space.id.get())
        .execute(&mut *tx)
        .await
        .map_err(|_| SpaceError::Storage)?;
    audit(
        &mut tx,
        &space,
        &input.source,
        space.member,
        "recovered",
        "pending",
    )
    .await?;
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(space)
}
