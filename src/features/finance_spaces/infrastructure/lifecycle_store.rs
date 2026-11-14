use super::super::{application::*, domain::*};
use super::{creation::audit, mapping, postgres::PgSpaceStore};
pub(super) async fn invalidate(
    store: &PgSpaceStore,
    guild: ServerRef,
    event: LifecycleEvent,
) -> Result<(), SpaceError> {
    let mut tx = store.pool.begin().await.map_err(|_| SpaceError::Storage)?;
    let (kind, target) = match event {
        LifecycleEvent::VerifyAll => ("all", None),
        LifecycleEvent::ChannelChanged(id) | LifecycleEvent::ChannelDeleted(id) => {
            ("channel", Some(id.get().to_string()))
        }
        LifecycleEvent::MemberRemoved(id) | LifecycleEvent::MemberChanged(id) => {
            ("member", Some(id.get().to_string()))
        }
    };
    let rows=sqlx::query("SELECT * FROM fg_finance_spaces WHERE guild_id=$1 AND ($2='all' OR ($2='channel' AND (channel_id=$3 OR category_id=$3)) OR ($2='member' AND (member_id=$3 OR id IN (SELECT space_id FROM fg_space_viewers WHERE member_id=$3)))) ORDER BY id FOR UPDATE")
        .bind(guild.get().to_string()).bind(kind).bind(&target).fetch_all(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
    for row in rows {
        let mut space = mapping::space(row)?;
        let retired = matches!(event,LifecycleEvent::MemberRemoved(id) if space.member==id)
            || matches!(event,LifecycleEvent::ChannelDeleted(id) if space.channel==Some(id));
        if space.state == SpaceState::Retired && space.current_operation.is_none() {
            continue;
        }
        if let LifecycleEvent::MemberRemoved(id) | LifecycleEvent::MemberChanged(id) = event {
            sqlx::query("UPDATE fg_space_viewers SET desired_state='revoked',confirmed_state='uncertain',revision=revision+1,changed_at=clock_timestamp() WHERE space_id=$1 AND member_id=$2 AND generation=$3 AND confirmed_state<>'absent'").bind(space.id.get()).bind(id.get().to_string()).bind(&space.generation).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
        }
        if retired {
            sqlx::query("UPDATE fg_space_operations SET phase='retired',completed_at=clock_timestamp(),outcome='retired' WHERE space_id=$1 AND phase NOT IN ('confirmed','failed','retired')").bind(space.id.get()).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
            space.state = SpaceState::Retired;
        } else if matches!(
            space.state,
            SpaceState::Ready | SpaceState::Verifying | SpaceState::Suspended
        ) {
            space.state = if matches!(event, LifecycleEvent::MemberRemoved(_)) {
                SpaceState::Suspended
            } else {
                SpaceState::Verifying
            };
        } else if !matches!(
            event,
            LifecycleEvent::MemberRemoved(_) | LifecycleEvent::MemberChanged(_)
        ) {
            continue;
        }
        space.revision = space.next_revision()?;
        sqlx::query("UPDATE fg_finance_spaces SET state=$1,revision=$2,current_operation_id=CASE WHEN $3 THEN NULL ELSE current_operation_id END,updated_at=clock_timestamp() WHERE id=$4")
            .bind(space.state.as_str()).bind(space.revision).bind(retired).bind(space.id.get()).execute(&mut *tx).await.map_err(|_|SpaceError::Storage)?;
        audit(
            &mut tx,
            &space,
            "lifecycle:invalidate",
            space.member,
            if retired { "retired" } else { "invalidated" },
            if retired { "retired" } else { "pending" },
        )
        .await?;
    }
    tx.commit().await.map_err(|_| SpaceError::Storage)?;
    Ok(())
}
pub(super) async fn check_legacy(
    store: &PgSpaceStore,
    input: LegacyCheck,
) -> Result<(), SpaceError> {
    let Some(owner) = input.owner else {
        return Ok(());
    };
    let conflict:bool=sqlx::query_scalar("SELECT EXISTS(SELECT FROM fg_finance_spaces WHERE (guild_id=$1 AND legacy_binding AND (member_id<>$2 OR journal_owner<>$3)) OR (guild_id=$1 AND member_id=$2 AND (NOT legacy_binding OR journal_owner<>$3)) OR (journal_owner=$3 AND (guild_id<>$1 OR member_id<>$2)))")
        .bind(input.guild.get().to_string()).bind(input.member.get().to_string()).bind(owner.as_str()).fetch_one(&store.pool).await.map_err(|_|SpaceError::Storage)?;
    if conflict {
        return Err(SpaceError::Conflict);
    }
    Ok(())
}
