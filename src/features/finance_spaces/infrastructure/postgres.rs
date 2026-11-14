use super::{
    super::{application::*, domain::*},
    mapping,
};
use sqlx::PgPool;

#[derive(Clone)]
pub(crate) struct PgSpaceStore {
    pub(super) pool: PgPool,
}
impl PgSpaceStore {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl SpaceStore for PgSpaceStore {
    fn viewers(&self, space: FinanceSpace) -> SpaceFuture<'_, Vec<ViewerGrant>> {
        Box::pin(async move {
            sqlx::query("SELECT * FROM fg_space_viewers WHERE space_id=$1 AND generation=$2 ORDER BY member_id").bind(space.id.get()).bind(&space.generation).fetch_all(&self.pool).await.map_err(|_|SpaceError::Storage)?.into_iter().map(mapping::viewer).collect()
        })
    }
    fn prepare_share(&self, input: SharingRequest) -> SpaceFuture<'_, SpaceOperation> {
        Box::pin(super::sharing_store::prepare(self, input))
    }
    fn expire_prepared(&self, id: SpaceId) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(super::sharing_store::expire(self, id))
    }
    fn cancel_prepared(&self, space: FinanceSpace, id: String) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(super::sharing_store::cancel(self, space, id))
    }

    fn invalidate(&self, guild: ServerRef, event: LifecycleEvent) -> SpaceFuture<'_, ()> {
        Box::pin(super::lifecycle_store::invalidate(self, guild, event))
    }
    fn tracked(&self, guild: ServerRef, after: i64) -> SpaceFuture<'_, Vec<FinanceSpace>> {
        Box::pin(async move {
            sqlx::query(
                "SELECT * FROM fg_finance_spaces WHERE guild_id=$1 AND id>$2 ORDER BY id LIMIT 2",
            )
            .bind(guild.get().to_string())
            .bind(after)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| SpaceError::Storage)?
            .into_iter()
            .map(mapping::space)
            .collect()
        })
    }
    fn recover(&self, input: RecoveryRequest) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(super::recovery_store::prepare(self, input))
    }
    fn check_legacy(&self, input: LegacyCheck) -> SpaceFuture<'_, ()> {
        Box::pin(super::lifecycle_store::check_legacy(self, input))
    }

    fn reserve(&self, input: Reservation) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(super::creation::reserve(self, input))
    }
    fn operation(&self, id: String) -> SpaceFuture<'_, SpaceOperation> {
        Box::pin(async move {
            let row = sqlx::query("SELECT * FROM fg_space_operations WHERE id=$1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| SpaceError::Storage)?
                .ok_or(SpaceError::Denied)?;
            mapping::operation(row)
        })
    }
    fn advance(
        &self,
        space: FinanceSpace,
        operation: SpaceOperation,
        step: OperationStep,
    ) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(super::operation_store::advance(
            self, space, operation, step,
        ))
    }

    fn find(&self, guild: ServerRef, member: MemberRef) -> SpaceFuture<'_, Option<FinanceSpace>> {
        Box::pin(async move {
            sqlx::query("SELECT * FROM fg_finance_spaces WHERE guild_id=$1 AND member_id=$2")
                .bind(guild.get().to_string())
                .bind(member.get().to_string())
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| SpaceError::Storage)?
                .map(mapping::space)
                .transpose()
        })
    }
    fn load(&self, id: SpaceId) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move {
            let row = sqlx::query("SELECT * FROM fg_finance_spaces WHERE id=$1")
                .bind(id.get())
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| SpaceError::Storage)?
                .ok_or(SpaceError::Denied)?;
            mapping::space(row)
        })
    }
    fn transition(&self, change: SpaceTransition) -> SpaceFuture<'_, FinanceSpace> {
        Box::pin(async move {
            change.validate()?;
            let mut tx = self.pool.begin().await.map_err(|_| SpaceError::Storage)?;
            let row=sqlx::query("UPDATE fg_finance_spaces SET state=$1,revision=revision+1,updated_at=clock_timestamp() WHERE id=$2 AND revision=$3 AND generation=$4 RETURNING *")
                .bind(change.state.as_str()).bind(change.space.get()).bind(change.expected_revision).bind(&change.generation)
                .fetch_optional(&mut *tx).await.map_err(|_| SpaceError::Storage)?.ok_or(SpaceError::Conflict)?;
            let updated = mapping::space(row)?;
            sqlx::query("INSERT INTO fg_space_access_events(space_id,operation_id,source,actor,action,outcome,transition_revision) VALUES($1,$2,$3,$4,$5,'pending',$6)")
                .bind(updated.id.get()).bind(&updated.current_operation).bind(&change.source).bind(change.actor.get().to_string())
                .bind(change.action.as_str()).bind(updated.revision).execute(&mut *tx).await.map_err(|_| SpaceError::Storage)?;
            tx.commit().await.map_err(|_| SpaceError::Storage)?;
            Ok(updated)
        })
    }
}
