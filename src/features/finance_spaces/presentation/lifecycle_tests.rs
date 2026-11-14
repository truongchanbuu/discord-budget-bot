use super::*;
use serenity::json::{from_value, json};
#[test]
fn sdk_membership_event_uses_authenticated_account_and_guild() {
    let event: GuildMemberRemoveEvent =
        from_value(json!({"guild_id":"10","user":{"id":"11","username":"spoofed-owner"}})).unwrap();
    assert_eq!(event.guild_id.get(), 10);
    assert_eq!(
        MemberRef::parse(&event.user.id.to_string()).unwrap(),
        MemberRef::parse("11").unwrap()
    );
}

#[tokio::test]
async fn sdk_disconnect_stops_finance_before_reconciliation() {
    let (spaces, owner, channel, bot) = ready_fixture().await;
    let handler = SpacesEvents {
        spaces: spaces.clone(),
        accepting: Arc::new(AtomicBool::new(true)),
        bot: Arc::new(AtomicU64::new(bot.get())),
        wake: Arc::new(tokio::sync::Notify::new()),
    };
    let event = serenity::gateway::ShardStageUpdateEvent {
        old: serenity::gateway::ConnectionStage::Connected,
        new: serenity::gateway::ConnectionStage::Disconnected,
        shard_id: ShardId(0),
    };
    handler.gateway_stage(event.new).await;
    assert!(!spaces.connected.load(Ordering::Acquire));
    assert!(spaces.authorize_finance(owner, channel, bot).await.is_err());
}

#[test]
fn sdk_role_events_preserve_guild_scope_for_invalidation() {
    let update: GuildRoleUpdateEvent = from_value(json!({
        "guild_id": "10",
        "role": {
            "id": "123",
            "name": "synthetic-role",
            "color": 0,
            "colors": {
                "primary_color": 0,
                "secondary_color": null,
                "tertiary_color": null
            },
            "hoist": false,
            "position": 1,
            "permissions": "8",
            "managed": false,
            "mentionable": false,
            "flags": 0
        }
    }))
    .unwrap();
    assert_eq!(update.role.guild_id.get(), 10);
    assert!(update.role.permissions.contains(Permissions::ADMINISTRATOR));
}
