use super::super::{application::*, domain::*};
use super::postgres::PgSpaceStore;
use crate::platform::database::test_support;

fn request(member: u64, source: &str) -> Reservation {
    Reservation {
        guild: ServerRef::parse("101").unwrap(),
        member: MemberRef::parse(&member.to_string()).unwrap(),
        category: ChannelRef::parse("103").unwrap(),
        joined_at: chrono::Utc::now(),
        legacy_owner: None,
        source: source.into(),
    }
}
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn concurrent_repeated_reservations_allocate_one_space_and_distinct_journals() {
    let store = PgSpaceStore::new(test_support::connect().await);
    let mut handles = Vec::new();
    for n in 0..10 {
        let store = store.clone();
        handles.push(tokio::spawn(async move {
            store
                .reserve(request(111, &format!("discord:guild:{n}")))
                .await
                .unwrap()
        }));
    }
    let first = handles.remove(0).await.unwrap();
    for handle in handles {
        let next = handle.await.unwrap();
        assert_eq!(next.id, first.id);
        assert_eq!(next.journal_owner, first.journal_owner);
    }
    let other = store
        .reserve(request(112, "discord:guild:other"))
        .await
        .unwrap();
    assert_ne!(other.journal_owner, first.journal_owner);
    let old = store.load(first.id).await.unwrap();
    assert_eq!(old.revision, 0);
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn guild_creation_quota_and_legacy_remap_are_durable() {
    let store = PgSpaceStore::new(test_support::connect().await);
    for n in 0..11 {
        let mut input = request(200 + n, &format!("quota:{n}"));
        input.guild = ServerRef::parse("102").unwrap();
        let result = store.reserve(input).await;
        if n < 10 {
            assert!(result.is_ok());
        } else {
            assert_eq!(result.unwrap_err(), SpaceError::RateLimited);
        }
    }
    let mut input = request(301, "legacy:first");
    input.guild = ServerRef::parse("104").unwrap();
    input.legacy_owner = Some(
        crate::features::financial_core::OwnerRef::new("synthetic-legacy-spaces".into()).unwrap(),
    );
    let original = store.reserve(input).await.unwrap();
    assert!(
        store
            .check_legacy(LegacyCheck {
                guild: original.guild,
                member: original.member,
                owner: Some(original.journal_owner.clone())
            })
            .await
            .is_ok()
    );
    assert_eq!(
        store
            .check_legacy(LegacyCheck {
                guild: original.guild,
                member: MemberRef::parse("302").unwrap(),
                owner: Some(original.journal_owner.clone())
            })
            .await,
        Err(SpaceError::Conflict)
    );

    let mut changed = request(301, "legacy:changed");
    changed.guild = original.guild;
    changed.legacy_owner =
        Some(crate::features::financial_core::OwnerRef::new("synthetic-remap".into()).unwrap());
    assert_eq!(
        store.reserve(changed).await.unwrap_err(),
        SpaceError::Conflict
    );
    assert_eq!(
        store.load(original.id).await.unwrap().journal_owner,
        original.journal_owner
    );
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn reservation_preserves_exact_membership_incarnation() {
    let store = PgSpaceStore::new(test_support::connect().await);
    let joined_at = "2026-11-14T00:00:00.123456789Z"
        .parse::<chrono::DateTime<chrono::Utc>>()
        .unwrap();
    let mut input = request(891, "precision:owner");
    input.guild = ServerRef::parse("890").unwrap();
    input.joined_at = joined_at;
    let saved = store.reserve(input).await.unwrap();
    assert_eq!(saved.owner_joined_at, Some(joined_at));
    assert_eq!(
        store.load(saved.id).await.unwrap().owner_joined_at,
        Some(joined_at)
    );
    let mut member = MemberSnapshot {
        member: saved.member,
        joined_at: Some(joined_at),
        human: true,
        pending: false,
        elevated: false,
    };
    assert!(verify_owner(&saved, &member).is_ok());
    member.joined_at = Some(joined_at + chrono::Duration::nanoseconds(1));
    assert_eq!(verify_owner(&saved, &member), Err(SpaceError::Denied));
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn concurrent_distinct_members_cannot_exceed_guild_creation_quota() {
    let store = PgSpaceStore::new(test_support::connect().await);
    let mut tasks = Vec::new();
    for id in 1..=12 {
        let store = store.clone();
        tasks.push(tokio::spawn(async move {
            let mut input = request(8000 + id, &format!("concurrent:{id}"));
            input.guild = ServerRef::parse("810").unwrap();
            store.reserve(input).await
        }));
    }
    let mut accepted = 0;
    let mut rejected = 0;
    for task in tasks {
        match task.await.unwrap() {
            Ok(_) => accepted += 1,
            Err(SpaceError::RateLimited) => rejected += 1,
            Err(error) => panic!("unexpected {error:?}"),
        }
    }
    assert_eq!((accepted, rejected), (10, 2));
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn repeated_retirement_cannot_bypass_member_creation_quota() {
    let store = PgSpaceStore::new(test_support::connect().await);
    let mut input = request(8201, "member:initial");
    input.guild = ServerRef::parse("820").unwrap();
    let mut space = store.reserve(input).await.unwrap();
    for attempt in 1..=3 {
        store
            .invalidate(space.guild, LifecycleEvent::MemberRemoved(space.member))
            .await
            .unwrap();
        space = store.load(space.id).await.unwrap();
        let result = store
            .recover(RecoveryRequest {
                space: space.id,
                expected_revision: space.revision,
                category: space.category,
                member: space.member,
                joined_at: chrono::Utc::now(),
                source: format!("member:retry:{attempt}"),
            })
            .await;
        if attempt < 3 {
            space = result.unwrap();
        } else {
            assert_eq!(result.unwrap_err(), SpaceError::RateLimited);
        }
    }
}
