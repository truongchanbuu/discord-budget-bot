use super::super::{application::*, domain::*};
use super::postgres::PgSpaceStore;
use crate::platform::database::test_support;

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn read_restore_and_stale_cas_have_one_atomic_audit() {
    let pool = test_support::connect().await;
    let owner: String = sqlx::query_scalar(
        "INSERT INTO fg_owners(owner) VALUES(gen_random_uuid()::text) RETURNING owner",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let id:i64=sqlx::query_scalar("INSERT INTO fg_finance_spaces(guild_id,member_id,journal_owner,generation,category_id,state) VALUES('14',(nextval('fg_finance_spaces_id_seq')+1000)::text,$1,gen_random_uuid()::text,'13','prepared') RETURNING id").bind(owner).fetch_one(&pool).await.unwrap();
    let store = PgSpaceStore::new(pool.clone());
    let original = store.load(SpaceId::new(id).unwrap()).await.unwrap();
    assert_eq!(
        store
            .find(original.guild, original.member)
            .await
            .unwrap()
            .unwrap()
            .id,
        original.id
    );
    let change = || SpaceTransition {
        space: original.id,
        expected_revision: 0,
        generation: original.generation.clone(),
        state: SpaceState::Suspended,
        source: "test:invalidate".into(),
        actor: original.member,
        action: AccessAction::Invalidated,
    };
    let updated = store.transition(change()).await.unwrap();
    assert_eq!(updated.revision, 1);
    assert_eq!(
        store.transition(change()).await.unwrap_err(),
        SpaceError::Conflict
    );
    let events: i64 =
        sqlx::query_scalar("SELECT count(*) FROM fg_space_access_events WHERE space_id=$1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(events, 1);
    let restored = PgSpaceStore::new(pool).load(original.id).await.unwrap();
    assert_eq!(restored.state, SpaceState::Suspended);
    assert_eq!(restored.journal_owner, original.journal_owner);
}
