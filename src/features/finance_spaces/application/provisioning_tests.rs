use super::super::domain::*;
use super::test_support::{MemoryDirectory, MemoryStore};
use super::*;
use crate::features::{financial_core::OwnerRef, identity::*};
use std::sync::Arc;

pub(super) fn fixture() -> (Spaces, Arc<MemoryStore>, Arc<MemoryDirectory>) {
    let store = Arc::new(MemoryStore::default());
    let directory = Arc::new(MemoryDirectory::default());
    let settings = SpaceSettings {
        guild: ServerRef::parse("10").unwrap(),
        category: ChannelRef::parse("13").unwrap(),
        onboarding: vec![ChannelRef::parse("14").unwrap()],
    };
    let binding = Arc::new(OwnerBinding::new(
        DiscordAccountId::parse("11").unwrap(),
        OwnerRef::new("legacy-owner".into()).unwrap(),
        BindingState::Enabled,
    ));
    let spaces = Spaces::new(settings, store.clone(), directory.clone(), binding);
    spaces
        .connected
        .store(true, std::sync::atomic::Ordering::Release);
    directory.members.lock().unwrap().push((
        spaces.settings.guild,
        MemberSnapshot {
            member: MemberRef::parse("11").unwrap(),
            joined_at: Some(chrono::Utc::now()),
            human: true,
            pending: false,
            elevated: false,
        },
    ));
    (spaces, store, directory)
}
#[tokio::test]
async fn uncertain_dispatch_is_not_repeated_and_foreign_onboarding_is_denied() {
    let (spaces, store, directory) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    directory
        .fail_create
        .store(true, std::sync::atomic::Ordering::Relaxed);
    assert!(
        spaces
            .onboard(
                owner,
                ChannelRef::parse("15").unwrap(),
                bot,
                "discord:guild:1"
            )
            .await
            .is_err()
    );
    assert_eq!(
        directory.creates.load(std::sync::atomic::Ordering::Relaxed),
        0
    );
    assert!(
        spaces
            .onboard(owner, spaces.settings.onboarding[0], bot, "discord:guild:2")
            .await
            .is_err()
    );
    assert!(
        spaces
            .onboard(owner, spaces.settings.onboarding[0], bot, "discord:guild:3")
            .await
            .is_err()
    );
    assert_eq!(
        directory.creates.load(std::sync::atomic::Ordering::Relaxed),
        1
    );
    let saved = store
        .find(spaces.settings.guild, owner)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(saved.state, SpaceState::Uncertain);
}
#[tokio::test]
async fn successful_onboarding_reuses_the_same_journal_and_channel() {
    let (spaces, _, directory) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let first = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "discord:guild:1")
        .await
        .unwrap();
    let second = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "discord:guild:2")
        .await
        .unwrap();
    assert_eq!(first.channel, second.channel);
    assert_eq!(first.journal_owner, second.journal_owner);
    assert_eq!(
        directory.creates.load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

pub(crate) async fn ready_fixture() -> (Spaces, MemberRef, ChannelRef, MemberRef) {
    let (spaces, _, directory) = fixture();
    directory.members.lock().unwrap().push((
        spaces.settings.guild,
        MemberSnapshot {
            member: MemberRef::parse("22").unwrap(),
            joined_at: Some(chrono::Utc::now()),
            human: true,
            pending: false,
            elevated: false,
        },
    ));
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let ready = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "test:ready")
        .await
        .unwrap();
    (spaces, owner, ready.channel.unwrap(), bot)
}

pub(crate) fn with_test_store(store: Arc<dyn SpaceStore>, guild: ServerRef) -> Spaces {
    let (mut spaces, _, directory) = fixture();
    spaces.store = store;
    spaces.settings = Arc::new(SpaceSettings {
        guild,
        category: spaces.settings.category,
        onboarding: spaces.settings.onboarding.clone(),
    });
    spaces.legacy = Arc::new(OwnerBinding::new(
        DiscordAccountId::parse("11").unwrap(),
        OwnerRef::new(format!("legacy-owner-{}", guild.get())).unwrap(),
        BindingState::Enabled,
    ));
    directory.members.lock().unwrap()[0].0 = guild;
    directory.members.lock().unwrap().push((
        spaces.settings.guild,
        MemberSnapshot {
            member: MemberRef::parse("22").unwrap(),
            joined_at: Some(chrono::Utc::now()),
            human: true,
            pending: false,
            elevated: false,
        },
    ));
    spaces
}
