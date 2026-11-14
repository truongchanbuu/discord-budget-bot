use super::super::{application::*, domain::*};
use super::postgres::PgSpaceStore;
use crate::platform::database::test_support;
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn retired_binding_preserves_journal_and_rejects_old_channel_adoption() {
    let store = PgSpaceStore::new(test_support::connect().await);
    let member = MemberRef::parse("601").unwrap();
    let mut space = store
        .reserve(Reservation {
            guild: ServerRef::parse("600").unwrap(),
            member,
            category: ChannelRef::parse("602").unwrap(),
            joined_at: chrono::Utc::now(),
            legacy_owner: None,
            source: "lifecycle:create".into(),
        })
        .await
        .unwrap();
    let old = ChannelRef::parse("6001").unwrap();
    for step in [
        OperationStep::Dispatch,
        OperationStep::Candidate(old),
        OperationStep::Confirm,
    ] {
        let op = store
            .operation(space.current_operation.clone().unwrap())
            .await
            .unwrap();
        space = store.advance(space, op, step).await.unwrap();
    }
    store
        .invalidate(space.guild, LifecycleEvent::MemberRemoved(member))
        .await
        .unwrap();
    let retired = store.load(space.id).await.unwrap();
    assert_eq!(retired.state, SpaceState::Retired);
    assert_eq!(retired.journal_owner, space.journal_owner);
    let new_incarnation = "2026-11-14T01:00:00.987654321Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();
    let recovery = store
        .recover(RecoveryRequest {
            space: retired.id,
            expected_revision: retired.revision,
            category: ChannelRef::parse("603").unwrap(),
            member,
            joined_at: new_incarnation,
            source: "lifecycle:recover".into(),
        })
        .await
        .unwrap();
    assert_eq!(recovery.journal_owner, space.journal_owner);
    assert_eq!(recovery.owner_joined_at, Some(new_incarnation));
    assert_ne!(recovery.generation, space.generation);
    assert!(recovery.channel.is_none());
    assert_eq!(recovery.category, ChannelRef::parse("603").unwrap());
    let op = store
        .operation(recovery.current_operation.clone().unwrap())
        .await
        .unwrap();
    let dispatched = store
        .advance(recovery, op, OperationStep::Dispatch)
        .await
        .unwrap();
    let op = store
        .operation(dispatched.current_operation.clone().unwrap())
        .await
        .unwrap();
    assert_eq!(
        store
            .advance(dispatched, op, OperationStep::Candidate(old))
            .await
            .unwrap_err(),
        SpaceError::Conflict
    );
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn durable_viewer_departure_removes_native_overwrite_without_regrant_on_add() {
    use std::sync::Arc;
    let store = Arc::new(PgSpaceStore::new(test_support::connect().await));
    let spaces = with_test_store(store.clone(), ServerRef::parse("902").unwrap());
    let member = MemberRef::parse("11").unwrap();
    let target = MemberRef::parse("22").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let ready = spaces
        .onboard(member, spaces.settings.onboarding[0], bot, "native:initial")
        .await
        .unwrap();
    let other = spaces
        .onboard(
            target,
            spaces.settings.onboarding[0],
            bot,
            "native:other-owner",
        )
        .await
        .unwrap();
    assert_ne!(other.journal_owner, ready.journal_owner);
    let channel = ready.channel.unwrap();
    let operation = spaces
        .prepare_viewer(
            ViewerRequest {
                member,
                channel,
                space: ready.id,
                target,
                kind: OperationKind::GrantViewer,
                source: "native:grant".into(),
            },
            bot,
        )
        .await
        .unwrap();
    spaces
        .confirm_viewer(
            member,
            channel,
            operation.id,
            operation.expected_revision,
            bot,
        )
        .await
        .unwrap();
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(target))
        .await
        .unwrap();
    assert_eq!(
        store.load(ready.id).await.unwrap().state,
        SpaceState::Suspended
    );
    spaces
        .reconcile(store.load(ready.id).await.unwrap(), bot)
        .await
        .unwrap();
    assert_eq!(
        store
            .viewers(store.load(ready.id).await.unwrap())
            .await
            .unwrap()[0]
            .confirmed,
        ViewerConfirmed::Absent
    );
    spaces
        .invalidate(LifecycleEvent::MemberChanged(target))
        .await
        .unwrap();
    spaces
        .reconcile(store.load(ready.id).await.unwrap(), bot)
        .await
        .unwrap();
    assert!(spaces.authorize_finance(member, channel, bot).await.is_ok());
    assert!(
        spaces
            .authorize_finance(target, channel, bot)
            .await
            .is_err()
    );
    let revokes: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM fg_space_operations WHERE space_id=$1 AND kind='revoke_viewer'",
    )
    .bind(ready.id.get())
    .fetch_one(&store.pool)
    .await
    .unwrap();
    assert_eq!(revokes, 1);
    let retained = store.load(other.id).await.unwrap();
    assert_eq!(retained.state, SpaceState::Retired);
    assert_eq!(retained.journal_owner, other.journal_owner);
    let audit:i64=sqlx::query_scalar("SELECT count(*) FROM fg_space_access_events WHERE space_id=$1 AND actor='99' AND action='revoked'").bind(ready.id.get()).fetch_one(&store.pool).await.unwrap();
    assert_eq!(audit, 1);
}
