use super::*;
use serenity::json::{from_value, json, to_value};
fn component(
    user: &str,
    custom: &str,
    kind: u8,
    values: Vec<&str>,
    channel: ChannelRef,
) -> ComponentInteraction {
    from_value(json!({"id":"100","application_id":"101","guild_id":"10","channel_id":channel.get().to_string(),
        "user":{"id":user,"username":"synthetic"},"data":{"custom_id":custom,"component_type":kind,"values":values},
        "token":"synthetic-interaction","version":1,"locale":"vi","entitlements":[],"attachment_size_limit":100,
        "message":{"id":"102","channel_id":channel.get().to_string(),"author":{"id":"99","username":"bot","bot":true},"content":"","timestamp":"2026-11-14T00:00:00Z","tts":false,"mention_everyone":false,"mentions":[],"mention_roles":[],"attachments":[],"embeds":[],"pinned":false,"type":0,"components":[]}})).unwrap()
}
#[tokio::test]
async fn sdk_select_requires_one_human_and_confirmation_stays_owner_only() {
    let (spaces, owner, channel, bot) = ready_fixture().await;
    let status = spaces.management_status(owner, channel).await.unwrap();
    let handler = SpacesEvents {
        spaces,
        accepting: Arc::new(AtomicBool::new(true)),
        bot: Arc::new(AtomicU64::new(bot.get())),
        wake: Arc::new(tokio::sync::Notify::new()),
    };
    let custom = format!(
        "fgspace:select_add:{}:{}",
        status.space.get(),
        status.revision
    );
    for (user, kind, values) in [
        ("22", 5, vec!["22"]),
        ("11", 3, vec!["22"]),
        ("11", 5, vec!["22", "23"]),
        ("11", 5, vec!["11"]),
    ] {
        assert!(
            components::action(&handler, &component(user, &custom, kind, values, channel))
                .await
                .is_err()
        );
    }
    let (warning, rows) =
        components::action(&handler, &component("11", &custom, 5, vec!["22"], channel))
            .await
            .unwrap();
    assert!(warning.contains("Administrator") && warning.contains("lịch sử"));
    let rows = to_value(rows).unwrap();
    let confirm = rows[0]["components"][0]["custom_id"].as_str().unwrap();
    assert!(confirm.len() <= 100);
    assert!(
        components::action(&handler, &component("22", confirm, 2, vec![], channel))
            .await
            .is_err()
    );
    assert!(
        components::action(&handler, &component("11", confirm, 2, vec![], channel))
            .await
            .is_ok()
    );
}
#[test]
fn malformed_foreign_or_oversized_control_references_are_rejected() {
    for value in [
        "fgfinance:manage:1:0",
        "fgspace:grant:1:0",
        "fgspace:manage:0:0",
        "fgspace:manage:1:-1",
        "fgspace:confirm:not-a-uuid:0",
    ] {
        assert!(components::parse(value).is_err());
    }
    assert!(components::parse(&format!("fgspace:confirm:{}:0", "a".repeat(101))).is_err());
}
