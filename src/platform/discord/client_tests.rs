use super::*;
use crate::features::{
    financial_core::OwnerRef,
    identity::{BindingState, DiscordAccountId, OwnerBinding},
};

fn data() -> CommandData {
    CommandData::new(
        OwnerBinding::new(
            DiscordAccountId::parse("34").unwrap(),
            OwnerRef::new("owner-marker".into()).unwrap(),
            BindingState::Enabled,
        ),
        serenity::GuildId::new(12),
    )
}

#[tokio::test]
async fn framework_table_and_permissions_are_narrow() {
    let (sender, _receiver) = oneshot::channel();
    let framework = framework(
        data(),
        serenity::GuildId::new(12),
        Instant::now() + Duration::from_secs(30),
        sender,
    );
    let options = framework.options();
    assert!(!options.initialize_owners);
    assert!(!options.skip_checks_for_owners);
    assert_eq!(options.commands.len(), 1);
    let command = &options.commands[0];
    assert_eq!(command.name, "fundguard");
    assert!(command.slash_action.is_some());
    assert!(command.prefix_action.is_none());
    assert!(command.context_menu_action.is_none());
    assert!(!command.owners_only);
    assert!(!command.guild_only);
    assert!(command.required_bot_permissions.is_empty());
    assert!(command.parameters.is_empty());
    assert!(command.subcommands.is_empty());
    assert!(
        options
            .commands
            .iter()
            .all(|command| command.name != "unsupported-command")
    );
    assert!(INTENTS.is_empty());
}

#[tokio::test]
async fn setup_result_is_signaled_independently_of_framework_user_data() {
    for result in [
        Ok(()),
        Err(SessionError::Registration),
        Err(SessionError::RegistrationTimeout),
    ] {
        let (sender, receiver) = oneshot::channel();
        let output = complete_setup(data(), sender, async { result }).await;
        assert_eq!(receiver.await.unwrap(), result);
        assert_eq!(output.is_ok(), result.is_ok());
    }
}
