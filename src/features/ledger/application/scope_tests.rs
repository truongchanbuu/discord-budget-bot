use super::*;
use crate::features::{
    finance_spaces::application::ready_fixture,
    identity::{BindingState, DiscordAccountId},
};
#[tokio::test]
async fn guild_authorization_ignores_forged_binding_and_denies_foreign_channel() {
    let (spaces, owner, channel, bot) = ready_fixture().await;
    let actor = TrustedActor::new(DiscordAccountId::parse(&owner.get().to_string()).unwrap());
    let forged = OwnerBinding::new(
        DiscordAccountId::parse("11").unwrap(),
        OwnerRef::new("forged-owner".into()).unwrap(),
        BindingState::Enabled,
    );
    let scope = Scope {
        actor: &actor,
        binding: &forged,
        channel: &channel.get().to_string(),
        guild: Some((spaces.settings.guild, bot)),
    };
    assert_eq!(
        resolve_scope(&scope, Some(&spaces)).await.unwrap().as_str(),
        "legacy-owner"
    );
    let other = Scope {
        channel: "999",
        ..scope
    };
    assert_eq!(
        resolve_scope(&other, Some(&spaces)).await.unwrap_err(),
        JournalError::Denied
    );
    assert!(resolve_scope(&other, None).await.is_err());
}
