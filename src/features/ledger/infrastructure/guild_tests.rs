use super::*;
use crate::{
    features::{
        finance_spaces::{
            MemberRef,
            application::{LifecycleEvent, with_test_store},
            infrastructure::PgSpaceStore,
        },
        identity::*,
    },
    platform::database::test_support,
};
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn two_guild_journals_preserve_exact_totals_replay_and_postcommit_revocation() {
    let pool = test_support::connect().await;
    let spaces = with_test_store(
        Arc::new(PgSpaceStore::new(pool.clone())),
        crate::features::finance_spaces::ServerRef::parse("10").unwrap(),
    );
    let owner = MemberRef::parse("11").unwrap();
    let other = MemberRef::parse("22").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let first = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "guild:a")
        .await
        .unwrap();
    let second = spaces
        .onboard(other, spaces.settings.onboarding[0], bot, "guild:b")
        .await
        .unwrap();
    assert_ne!(first.journal_owner, second.journal_owner);
    let journal = PgStore::journal(pool.clone()).with_spaces(spaces.clone());
    let actor = TrustedActor::new(DiscordAccountId::parse("11").unwrap());
    let other_actor = TrustedActor::new(DiscordAccountId::parse("22").unwrap());
    let untrusted = OwnerBinding::new(
        DiscordAccountId::parse("11").unwrap(),
        OwnerRef::new("forged".into()).unwrap(),
        BindingState::Enabled,
    );
    let channel = first.channel.unwrap().get().to_string();
    let other_channel = second.channel.unwrap().get().to_string();
    let scope = || Scope {
        actor: &actor,
        binding: &untrusted,
        channel: &channel,
        guild: Some((spaces.settings.guild, bot)),
    };
    let foreign = || Scope {
        actor: &other_actor,
        binding: &untrusted,
        channel: &other_channel,
        guild: Some((spaces.settings.guild, bot)),
    };
    for (source, input) in [
        ("income", "nhận lương 16tr"),
        ("expense", "chi 6tr ăn uống"),
        ("saving", "tiết kiệm 3tr"),
        ("investment", "đầu tư 2tr"),
    ] {
        let draft = journal
            .preview(scope(), source, input, chrono::Utc::now())
            .await
            .unwrap();
        journal
            .act(scope(), draft.id, draft.revision, Action::Confirm)
            .await
            .unwrap();
        let replay = journal
            .act(scope(), draft.id, draft.revision, Action::Confirm)
            .await
            .unwrap();
        assert!(matches!(
            replay,
            ActionResult::Committed(Receipt { replayed: true, .. })
        ));
        assert!(
            journal
                .act(foreign(), draft.id, draft.revision, Action::Confirm)
                .await
                .is_err()
        );
    }
    let month = Month::current(local_date(chrono::Utc::now())).unwrap();
    let report = journal.summary(scope(), month).await.unwrap();
    assert_eq!(
        (
            report.income,
            report.expenses,
            report.closing,
            report.available
        ),
        (16_000_000, 6_000_000, 10_000_000, 5_000_000)
    );
    assert_eq!(journal.summary(foreign(), month).await.unwrap().closing, 0);
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(owner))
        .await
        .unwrap();
    assert!(journal.summary(scope(), month).await.is_err());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM fg_entries WHERE owner=$1")
        .bind(first.journal_owner.as_str())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 4);
    let restarted = PgStore::journal(pool).with_spaces(spaces.clone());
    assert_eq!(
        restarted.summary(foreign(), month).await.unwrap().closing,
        0
    );
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn existing_legacy_dm_entries_remain_owned_after_guild_onboarding() {
    let pool = test_support::connect().await;
    let spaces = with_test_store(
        Arc::new(PgSpaceStore::new(pool.clone())),
        crate::features::finance_spaces::ServerRef::parse("903").unwrap(),
    );
    let account = DiscordAccountId::parse("11").unwrap();
    let actor = TrustedActor::new(account);
    let binding = OwnerBinding::new(
        account,
        OwnerRef::new("legacy-owner-903".into()).unwrap(),
        BindingState::Enabled,
    );
    let dm = || Scope {
        actor: &actor,
        binding: &binding,
        channel: "9039",
        guild: None,
    };
    let journal = PgStore::journal(pool.clone());
    let draft = journal
        .preview(dm(), "discord:dm:903", "nhận lương 2tr", chrono::Utc::now())
        .await
        .unwrap();
    journal
        .act(dm(), draft.id, draft.revision, Action::Confirm)
        .await
        .unwrap();
    let member = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let ready = spaces
        .onboard(member, spaces.settings.onboarding[0], bot, "legacy:guild")
        .await
        .unwrap();
    assert_eq!(ready.journal_owner.as_str(), "legacy-owner-903");
    let channel = ready.channel.unwrap().get().to_string();
    let guild = || Scope {
        actor: &actor,
        binding: &binding,
        channel: &channel,
        guild: Some((spaces.settings.guild, bot)),
    };
    let connected = PgStore::journal(pool).with_spaces(spaces.clone());
    let month = Month::current(local_date(chrono::Utc::now())).unwrap();
    assert_eq!(
        connected.summary(guild(), month).await.unwrap().closing,
        2_000_000
    );
    assert_eq!(
        connected.summary(dm(), month).await.unwrap().closing,
        2_000_000
    );
    assert!(
        connected
            .act(guild(), draft.id, draft.revision, Action::Confirm)
            .await
            .is_err()
    );
}
