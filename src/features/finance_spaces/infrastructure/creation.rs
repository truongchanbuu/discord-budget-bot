use super::super::{application::*, domain::*};
use super::{mapping, postgres::PgSpaceStore};
use sqlx::{Postgres, Transaction};

pub(super) async fn quota(
    tx: &mut Transaction<'_, Postgres>,
    guild: ServerRef,
    member: MemberRef,
) -> Result<(), SpaceError> {
    let (guild_attempts,member_attempts):(i64,i64)=sqlx::query_as("SELECT count(*) FILTER(WHERE o.created_at>clock_timestamp()-INTERVAL '1 minute'),count(*) FILTER(WHERE s.member_id=$2) FROM fg_space_operations o JOIN fg_finance_spaces s ON s.id=o.space_id WHERE s.guild_id=$1 AND o.kind='create' AND o.created_at>clock_timestamp()-INTERVAL '10 minutes'")
        .bind(guild.get().to_string()).bind(member.get().to_string()).fetch_one(&mut **tx).await.map_err(|_|SpaceError::Storage)?;
    if guild_attempts >= 10 || member_attempts >= 3 {
        return Err(SpaceError::RateLimited);
    }
    let count:i64=sqlx::query_scalar("SELECT count(*) FROM fg_finance_spaces WHERE guild_id=$1 AND (state<>'retired' OR current_operation_id IS NOT NULL)").bind(guild.get().to_string()).fetch_one(&mut **tx).await.map_err(|_|SpaceError::Storage)?;
    if count >= 50 {
        return Err(SpaceError::Capacity);
    }
    Ok(())
}
pub(super) async fn guild_lock(
    tx: &mut Transaction<'_, Postgres>,
    guild: ServerRef,
) -> Result<(), SpaceError> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))")
        .bind(guild.get().to_string())
        .execute(&mut **tx)
        .await
        .map_err(|_| SpaceError::Storage)?;
    Ok(())
}
pub(super) async fn audit(
    tx: &mut Transaction<'_, Postgres>,
    space: &FinanceSpace,
    source: &str,
    actor: MemberRef,
    action: &str,
    outcome: &str,
) -> Result<(), SpaceError> {
    let event = AccessEvent {
        space: space.id,
        operation: space.current_operation.clone(),
        source: source.into(),
        actor,
        action: AccessAction::parse(action)?,
        outcome: OperationOutcome::parse(outcome)?,
        at: chrono::Utc::now(),
    };
    event.validate()?;
    sqlx::query("INSERT INTO fg_space_access_events(space_id,operation_id,source,actor,action,outcome,transition_revision,created_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8)")
        .bind(event.space.get()).bind(&event.operation).bind(&event.source).bind(event.actor.get().to_string())
        .bind(event.action.as_str()).bind(event.outcome.as_str()).bind(space.revision).bind(event.at)
        .execute(&mut **tx).await.map_err(|_|SpaceError::Storage)?;
    Ok(())
}
pub(super) async fn reserve(
    store: &PgSpaceStore,
    input: Reservation,
) -> Result<FinanceSpace, SpaceError> {
    input.validate()?;
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    guild_lock(&mut tx, input.guild).await?;
    if let Some(row) =
        sqlx::query("SELECT * FROM fg_finance_spaces WHERE guild_id=$1 AND member_id=$2 FOR UPDATE")
            .bind(input.guild.get().to_string())
            .bind(input.member.get().to_string())
            .fetch_optional(&mut *tx)
            .await
            .map_err(|_| SpaceError::Storage)?
    {
        let space = mapping::space(row)?;
        if space.legacy != input.legacy_owner.is_some()
            || input
                .legacy_owner
                .as_ref()
                .is_some_and(|owner| owner != &space.journal_owner)
        {
            return Err(SpaceError::Conflict);
        }
        return Ok(space);
    }
    quota(&mut tx, input.guild, input.member).await?;
    let legacy = input.legacy_owner.is_some();
    let owner = if let Some(owner) = input.legacy_owner {
        sqlx::query("INSERT INTO fg_owners(owner) VALUES($1) ON CONFLICT DO NOTHING")
            .bind(owner.as_str())
            .execute(&mut *tx)
            .await
            .map_err(|_| SpaceError::Storage)?;
        owner.as_str().to_owned()
    } else {
        sqlx::query_scalar::<_, String>(
            "INSERT INTO fg_owners(owner) VALUES(gen_random_uuid()::text) RETURNING owner",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| SpaceError::Storage)?
    };
    let row=sqlx::query("INSERT INTO fg_finance_spaces(guild_id,member_id,journal_owner,legacy_binding,owner_joined_at,generation,category_id,state) VALUES($1,$2,$3,$4,$5,gen_random_uuid()::text,$6,'prepared') RETURNING *")
        .bind(input.guild.get().to_string()).bind(input.member.get().to_string()).bind(owner).bind(legacy).bind(super::incarnation::encode(input.joined_at)).bind(input.category.get().to_string()).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Conflict)?;
    let mut space = mapping::space(row)?;
    let op:String=sqlx::query_scalar("INSERT INTO fg_space_operations(space_id,kind,source,actor,generation,expected_revision,phase) VALUES($1,'create',$2,$3,$4,0,'prepared') RETURNING id").bind(space.id.get()).bind(&input.source).bind(input.member.get().to_string()).bind(&space.generation).fetch_one(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    sqlx::query("UPDATE fg_finance_spaces SET current_operation_id=$2 WHERE id=$1")
        .bind(space.id.get())
        .bind(&op)
        .execute(&mut *tx)
        .await
        .map_err(|_| SpaceError::Storage)?;
    space.current_operation = Some(op);
    audit(
        &mut tx,
        &space,
        &input.source,
        input.member,
        "reserved",
        "pending",
    )
    .await?;
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(space)
}
