use super::super::{application::*, domain::*};
use super::postgres::PgSpaceStore;
use crate::platform::database::test_support;
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn prepared_expiry_and_dispatched_protection_are_atomic_and_durable() {
    let store = PgSpaceStore::new(test_support::connect().await);
    let mut space = store
        .reserve(Reservation {
            guild: ServerRef::parse("700").unwrap(),
            member: MemberRef::parse("701").unwrap(),
            category: ChannelRef::parse("702").unwrap(),
            joined_at: chrono::Utc::now(),
            legacy_owner: None,
            source: "sharing:create".into(),
        })
        .await
        .unwrap();
    for step in [
        OperationStep::Dispatch,
        OperationStep::Candidate(ChannelRef::parse("7001").unwrap()),
        OperationStep::Confirm,
    ] {
        let op = store
            .operation(space.current_operation.clone().unwrap())
            .await
            .unwrap();
        space = store.advance(space, op, step).await.unwrap();
    }
    let target_incarnation = "2026-11-14T02:00:00.123456789Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();
    let input = |space: &FinanceSpace, source: &str| SharingRequest {
        authority: SharingAuthority::Owner,
        space: space.id,
        revision: space.revision,
        generation: space.generation.clone(),
        actor: space.member,
        target: MemberRef::parse("703").unwrap(),
        joined_at: target_incarnation,
        kind: OperationKind::GrantViewer,
        source: source.into(),
    };
    let op = store
        .prepare_share(input(&space, "sharing:abandoned"))
        .await
        .unwrap();
    sqlx::query("UPDATE fg_space_operations SET created_at=clock_timestamp()-INTERVAL '6 minutes' WHERE id=$1").bind(&op.id).execute(&store.pool).await.unwrap();
    let expired = store.expire_prepared(space.id).await.unwrap();
    store.expire_prepared(space.id).await.unwrap();
    assert!(expired.current_operation.is_none());
    assert_eq!(expired.state, SpaceState::Ready);
    assert!(store.viewers(expired.clone()).await.unwrap().is_empty());
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM fg_space_access_events WHERE operation_id=$1 AND action='expired'",
    )
    .bind(&op.id)
    .fetch_one(&store.pool)
    .await
    .unwrap();
    assert_eq!(count, 1);
    let op = store
        .prepare_share(input(&expired, "sharing:live"))
        .await
        .unwrap();
    assert_eq!(
        store
            .operation(op.id.clone())
            .await
            .unwrap()
            .target_joined_at,
        Some(target_incarnation)
    );
    let prepared = store.load(space.id).await.unwrap();
    let dispatched = store
        .advance(prepared, op.clone(), OperationStep::Dispatch)
        .await
        .unwrap();
    sqlx::query("UPDATE fg_space_operations SET created_at=clock_timestamp()-INTERVAL '6 minutes' WHERE id=$1").bind(&op.id).execute(&store.pool).await.unwrap();
    let retained = store.expire_prepared(space.id).await.unwrap();
    assert_eq!(retained.current_operation, Some(op.id.clone()));
    assert!(
        store
            .prepare_share(input(&retained, "sharing:opposite"))
            .await
            .is_err()
    );
    let op = store.operation(op.id).await.unwrap();
    let verified = store
        .advance(
            dispatched,
            op,
            OperationStep::Candidate(ChannelRef::parse("7001").unwrap()),
        )
        .await
        .unwrap();
    let op = store
        .operation(verified.current_operation.clone().unwrap())
        .await
        .unwrap();
    let ready = store
        .advance(verified, op, OperationStep::Confirm)
        .await
        .unwrap();
    let viewers = store.viewers(ready).await.unwrap();
    assert_eq!(viewers[0].confirmed, ViewerConfirmed::Viewer);
    assert_eq!(viewers[0].joined_at, target_incarnation);
}
