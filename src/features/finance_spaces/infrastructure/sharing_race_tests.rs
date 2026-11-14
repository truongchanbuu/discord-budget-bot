use super::{
    super::{application::*, domain::*},
    postgres::PgSpaceStore,
};
use crate::platform::database::test_support;
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn confirmation_and_expiry_serialize_with_no_expired_dispatch() {
    let store = PgSpaceStore::new(test_support::connect().await);
    for (guild, expired) in [(850, false), (851, true)] {
        let mut space = store
            .reserve(Reservation {
                guild: ServerRef::parse(&guild.to_string()).unwrap(),
                member: MemberRef::parse("8501").unwrap(),
                category: ChannelRef::parse("8502").unwrap(),
                joined_at: chrono::Utc::now(),
                legacy_owner: None,
                source: "race:create".into(),
            })
            .await
            .unwrap();
        let channel = ChannelRef::parse(&(85000 + guild).to_string()).unwrap();
        for step in [
            OperationStep::Dispatch,
            OperationStep::Candidate(channel),
            OperationStep::Confirm,
        ] {
            let op = store
                .operation(space.current_operation.clone().unwrap())
                .await
                .unwrap();
            space = store.advance(space, op, step).await.unwrap();
        }
        let operation = store
            .prepare_share(SharingRequest {
                authority: SharingAuthority::Owner,
                space: space.id,
                revision: space.revision,
                generation: space.generation.clone(),
                actor: space.member,
                target: MemberRef::parse("8503").unwrap(),
                joined_at: chrono::Utc::now(),
                kind: OperationKind::GrantViewer,
                source: "race:grant".into(),
            })
            .await
            .unwrap();
        let statement = if expired {
            "UPDATE fg_space_operations SET created_at=clock_timestamp()-INTERVAL '6 minutes' WHERE id=$1"
        } else {
            "UPDATE fg_space_operations SET created_at=clock_timestamp()-INTERVAL '4 minutes' WHERE id=$1"
        };
        sqlx::query(statement)
            .bind(&operation.id)
            .execute(&store.pool)
            .await
            .unwrap();
        let current = store.load(space.id).await.unwrap();
        let (cleanup, dispatch) = tokio::join!(
            store.expire_prepared(space.id),
            store.advance(current, operation.clone(), OperationStep::Dispatch)
        );
        cleanup.unwrap();
        let saved = store.load(space.id).await.unwrap();
        let op = store.operation(operation.id.clone()).await.unwrap();
        if expired {
            assert!(dispatch.is_err());
            assert!(saved.current_operation.is_none());
            assert_eq!(op.outcome, Some(OperationOutcome::Expired));
            assert!(store.viewers(saved).await.unwrap().is_empty());
        } else {
            assert!(dispatch.is_ok());
            assert_eq!(op.phase, OperationPhase::Dispatched);
            assert!(saved.current_operation.is_some());
        }
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM fg_space_access_events WHERE operation_id=$1 AND action='expired'").bind(&operation.id).fetch_one(&store.pool).await.unwrap();
        assert_eq!(count, if expired { 1 } else { 0 });
    }
}
