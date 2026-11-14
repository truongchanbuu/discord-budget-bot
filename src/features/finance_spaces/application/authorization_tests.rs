use super::provisioning_tests::fixture;
use super::*;
#[tokio::test]
async fn finance_checks_owner_channel_incarnation_and_nonready_management() {
    let (spaces, store, directory) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let ready = spaces
        .onboard(owner, spaces.settings.onboarding[0], bot, "auth:start")
        .await
        .unwrap();
    assert!(
        spaces
            .authorize_finance(owner, ready.channel.unwrap(), bot)
            .await
            .is_ok()
    );
    assert!(
        spaces
            .authorize_finance(MemberRef::parse("22").unwrap(), ready.channel.unwrap(), bot)
            .await
            .is_err()
    );
    assert!(
        spaces
            .authorize_finance(owner, spaces.settings.onboarding[0], bot)
            .await
            .is_err()
    );
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(owner))
        .await
        .unwrap();
    assert!(
        spaces
            .authorize_finance(owner, ready.channel.unwrap(), bot)
            .await
            .is_err()
    );
    assert!(
        spaces
            .authorize_owner_management(owner, spaces.settings.onboarding[0], ready.id)
            .await
            .is_ok()
    );
    assert!(
        spaces
            .authorize_owner_management(owner, ready.channel.unwrap(), ready.id)
            .await
            .is_err()
    );
    let stored = store.load(ready.id).await.unwrap();
    assert_eq!(stored.journal_owner, ready.journal_owner);
    directory.members.lock().unwrap()[0].1.joined_at = None;
    assert!(
        spaces
            .authorize_owner_management(owner, spaces.settings.onboarding[0], ready.id)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn changing_disabled_configuration_cannot_recover_an_existing_legacy_association() {
    let (mut spaces, store, _) = fixture();
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let old = spaces
        .onboard(
            owner,
            spaces.settings.onboarding[0],
            bot,
            "legacy:configured",
        )
        .await
        .unwrap();
    spaces
        .invalidate(LifecycleEvent::MemberRemoved(owner))
        .await
        .unwrap();
    spaces.legacy = std::sync::Arc::new(crate::features::identity::OwnerBinding::new(
        crate::features::identity::DiscordAccountId::parse("12").unwrap(),
        crate::features::financial_core::OwnerRef::new("different-owner".into()).unwrap(),
        crate::features::identity::BindingState::Disabled,
    ));
    assert!(
        spaces
            .authorize_owner_management(owner, spaces.settings.onboarding[0], old.id)
            .await
            .is_err()
    );
    assert!(
        spaces
            .request_recovery(owner, spaces.settings.onboarding[0], "legacy:remap")
            .await
            .is_err()
    );
    assert_eq!(
        store.load(old.id).await.unwrap().journal_owner,
        old.journal_owner
    );
}
