use super::{
    super::{application::*, domain::*},
    postgres::PgSpaceStore,
};
use crate::platform::database::test_support;
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn capacity_counts_pending_slots_and_viewer_changes_enforce_both_limits() {
    let pool = test_support::connect().await;
    let store = PgSpaceStore::new(pool.clone());
    sqlx::query("WITH owners AS (INSERT INTO fg_owners(owner) SELECT gen_random_uuid()::text FROM generate_series(1,50) RETURNING owner) INSERT INTO fg_finance_spaces(guild_id,member_id,journal_owner,generation,category_id,state) SELECT '840',(84000+row_number() OVER())::text,owner,gen_random_uuid()::text,'8401','prepared' FROM owners").execute(&pool).await.unwrap();
    let full = store
        .reserve(Reservation {
            guild: ServerRef::parse("840").unwrap(),
            member: MemberRef::parse("99999").unwrap(),
            category: ChannelRef::parse("8401").unwrap(),
            joined_at: chrono::Utc::now(),
            legacy_owner: None,
            source: "limits:capacity".into(),
        })
        .await;
    assert_eq!(full.unwrap_err(), SpaceError::Capacity);
    let mut space = store
        .reserve(Reservation {
            guild: ServerRef::parse("841").unwrap(),
            member: MemberRef::parse("8411").unwrap(),
            category: ChannelRef::parse("8412").unwrap(),
            joined_at: chrono::Utc::now(),
            legacy_owner: None,
            source: "limits:initial".into(),
        })
        .await
        .unwrap();
    let channel = ChannelRef::parse("8413").unwrap();
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
    let request =
        |space: &FinanceSpace, target: u64, kind: OperationKind, source: String| SharingRequest {
            authority: SharingAuthority::Owner,
            space: space.id,
            revision: space.revision,
            generation: space.generation.clone(),
            actor: space.member,
            target: MemberRef::parse(&target.to_string()).unwrap(),
            joined_at: chrono::Utc::now(),
            kind,
            source,
        };
    for n in 0..8 {
        store
            .prepare_share(request(
                &space,
                8420 + n,
                OperationKind::GrantViewer,
                format!("limits:grant:{n}"),
            ))
            .await
            .unwrap();
        space = store.load(space.id).await.unwrap();
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
    }
    assert_eq!(
        store
            .prepare_share(request(
                &space,
                99990,
                OperationKind::GrantViewer,
                "limits:ninth".into()
            ))
            .await
            .unwrap_err(),
        SpaceError::Capacity
    );
    for (kind, source) in [
        (OperationKind::RevokeViewer, "limits:revoke"),
        (OperationKind::GrantViewer, "limits:regrant"),
    ] {
        store
            .prepare_share(request(&space, 8420, kind, source.into()))
            .await
            .unwrap();
        space = store.load(space.id).await.unwrap();
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
    }
    assert_eq!(
        store
            .prepare_share(request(
                &space,
                8420,
                OperationKind::RevokeViewer,
                "limits:eleventh".into()
            ))
            .await
            .unwrap_err(),
        SpaceError::RateLimited
    );
}
