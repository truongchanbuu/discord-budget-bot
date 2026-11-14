use super::{provisioning_tests::fixture, *};

#[tokio::test]
async fn unrestricted_onboarding_allows_new_channels_but_not_public_finance() {
    let (mut spaces, store, _) = fixture();
    std::sync::Arc::get_mut(&mut spaces.settings)
        .unwrap()
        .onboarding
        .clear();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let channel = ChannelRef::parse("12345").unwrap();
    let ready = spaces
        .onboard(owner, channel, bot, "auto:start")
        .await
        .unwrap();
    assert!(spaces.authorize_finance(owner, channel, bot).await.is_err());
    assert!(
        spaces
            .authorize_finance(owner, ready.channel.unwrap(), bot)
            .await
            .is_ok()
    );
    spaces
        .invalidate(LifecycleEvent::ChannelDeleted(ready.channel.unwrap()))
        .await
        .unwrap();
    let prepared = spaces
        .request_recovery(owner, channel, "auto:recover")
        .await
        .unwrap();
    let recovered = spaces
        .confirm_recovery(owner, channel, prepared.id, prepared.revision, bot)
        .await
        .unwrap();
    assert_eq!(recovered.journal_owner, ready.journal_owner);
    assert_ne!(recovered.channel, ready.channel);
    assert_eq!(store.load(ready.id).await.unwrap().state, SpaceState::Ready);
}

#[test]
fn onboarding_allowlist_remains_an_explicit_restriction() {
    let (spaces, _, _) = fixture();
    assert!(
        spaces
            .settings
            .permits_onboarding(ChannelRef::parse("14").unwrap())
    );
    assert!(
        !spaces
            .settings
            .permits_onboarding(ChannelRef::parse("15").unwrap())
    );
}
