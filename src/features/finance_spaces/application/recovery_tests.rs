use super::provisioning_tests::fixture;
use super::*;
#[tokio::test]
async fn owner_departure_retires_binding_without_deleting_journal() {
    let (spaces, store, _) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let ready = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "lifecycle:start")
        .await
        .unwrap();
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(owner))
        .await
        .unwrap();
    let retired = store.load(ready.id).await.unwrap();
    assert_eq!(retired.state, SpaceState::Retired);
    assert_eq!(retired.journal_owner, ready.journal_owner);
    assert_ne!(retired.state, SpaceState::Ready);
}

#[tokio::test]
async fn fresh_recovery_uses_current_category_and_rejects_old_controls() {
    let (mut spaces, store, directory) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let old = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "recovery:old")
        .await
        .unwrap();
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(owner))
        .await
        .unwrap();
    let new_join =
        directory.members.lock().unwrap()[0].1.joined_at.unwrap() + chrono::Duration::seconds(1);
    directory.members.lock().unwrap()[0].1.joined_at = Some(new_join);
    spaces.settings = std::sync::Arc::new(SpaceSettings {
        guild: spaces.settings.guild,
        category: ChannelRef::parse("77").unwrap(),
        onboarding: spaces.settings.onboarding.clone(),
    });
    let prepared = spaces
        .request_recovery(owner, spaces.settings.onboarding[0], "recovery:new")
        .await
        .unwrap();
    assert_eq!(prepared.category, spaces.settings.category);
    assert!(
        spaces
            .confirm_recovery(
                owner,
                spaces.settings.onboarding[0],
                old.id,
                old.revision,
                bot
            )
            .await
            .is_err()
    );
    let ready = spaces
        .confirm_recovery(
            owner,
            spaces.settings.onboarding[0],
            prepared.id,
            prepared.revision,
            bot,
        )
        .await
        .unwrap();
    assert_ne!(ready.channel, old.channel);
    assert_ne!(ready.generation, old.generation);
    assert_eq!(ready.journal_owner, old.journal_owner);
    assert_eq!(ready.owner_joined_at, Some(new_join));
    assert!(store.viewers(ready).await.unwrap().is_empty());
}

#[tokio::test]
async fn known_deleted_channel_recovers_but_unknown_visibility_does_not() {
    for missing in [false, true] {
        let (spaces, store, directory) = fixture();
        let owner = MemberRef::parse("11").unwrap();
        let bot = MemberRef::parse("99").unwrap();
        let ready = spaces
            .onboard(owner, spaces.settings.onboarding[0], bot, "deleted:initial")
            .await
            .unwrap();
        if missing {
            directory
                .missing
                .lock()
                .unwrap()
                .push(ready.channel.unwrap());
        } else {
            directory
                .fail_inspect
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        assert!(
            spaces
                .authorize_finance(owner, ready.channel.unwrap(), bot)
                .await
                .is_err()
        );
        let saved = store.load(ready.id).await.unwrap();
        if missing {
            assert_eq!(saved.state, SpaceState::Retired);
            assert!(
                spaces
                    .request_recovery(owner, spaces.settings.onboarding[0], "deleted:request")
                    .await
                    .is_ok()
            );
        } else {
            assert_ne!(saved.state, SpaceState::Retired);
            assert!(
                spaces
                    .request_recovery(owner, spaces.settings.onboarding[0], "hidden:request")
                    .await
                    .is_err()
            );
        }
        assert_eq!(
            directory.creates.load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }
}
