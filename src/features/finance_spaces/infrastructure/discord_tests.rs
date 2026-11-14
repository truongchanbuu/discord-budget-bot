use super::discord::*;
use serenity::{
    builder::CreateChannel,
    json::{from_value, json, to_value},
    model::prelude::*,
};

#[test]
fn sdk_private_profile_and_thread_denial_are_exact() {
    let owner = UserId::new(11);
    let bot = UserId::new(99);
    let guild = GuildId::new(10);
    let overwrites = canonical_overwrites(guild, owner, bot, &[UserId::new(22)]);
    assert_eq!(overwrites.len(), 4);
    assert_eq!(overwrites[0].deny, Permissions::VIEW_CHANNEL);
    let viewer = overwrites.last().unwrap();
    assert!(viewer.deny.contains(
        Permissions::SEND_MESSAGES
            | Permissions::CREATE_PUBLIC_THREADS
            | Permissions::CREATE_PRIVATE_THREADS
            | Permissions::SEND_MESSAGES_IN_THREADS
    ));
    assert!(
        overwrites
            .iter()
            .all(|o| !o.allow.contains(Permissions::MANAGE_ROLES)
                && !o.deny.contains(Permissions::ADMINISTRATOR))
    );
    let value = to_value(
        CreateChannel::new("private-finance")
            .kind(ChannelType::Text)
            .permissions(overwrites),
    )
    .unwrap();
    assert_eq!(value["permission_overwrites"].as_array().unwrap().len(), 4);
}

fn message(content: &str, mentions: bool) -> Message {
    from_value(json!({"id":"100","channel_id":"12","guild_id":"10","author":{"id":"11","username":"synthetic"},
      "content":content,"timestamp":"2026-11-14T00:00:00Z","edited_timestamp":null,"tts":false,
      "mention_everyone":false,"mentions":if mentions {vec![json!({"id":"99","username":"bot","bot":true})]} else {vec![]},
      "mention_roles":[],"attachments":[],"embeds":[],"pinned":false,"type":0,"components":[]})).unwrap()
}

#[test]
fn sdk_mention_metadata_and_leading_token_are_both_required() {
    for content in ["<@99> bắt đầu", "<@!99> bắt đầu"] {
        assert_eq!(
            mentioned_input(&message(content, true), UserId::new(99)),
            Some("bắt đầu")
        );
    }
    for (content, mentions) in [
        ("reply only", true),
        ("<@99> bắt đầu", false),
        ("quoted <@99>", true),
        ("<@&99> bắt đầu", true),
        ("<@99>suffix", true),
        ("<@98> bắt đầu", true),
    ] {
        assert!(mentioned_input(&message(content, mentions), UserId::new(99)).is_none());
    }
}

#[test]
fn actual_sdk_masks_normalize_without_ignoring_extra_bits() {
    use super::super::domain::{AccessProfile, Subject};
    use super::inspection::normalize_overwrite;
    let guild = GuildId::new(10);
    let mut entries =
        canonical_overwrites(guild, UserId::new(11), UserId::new(99), &[UserId::new(22)]);
    assert!(normalize_overwrite(&entries[0], guild).profile == AccessProfile::Hidden);
    assert!(normalize_overwrite(&entries[3], guild).profile == AccessProfile::Viewer);
    entries[3].allow |= Permissions::SEND_MESSAGES;
    assert!(normalize_overwrite(&entries[3], guild).profile == AccessProfile::Other);
    entries[0].kind = PermissionOverwriteType::Role(RoleId::new(123));
    assert!(normalize_overwrite(&entries[0], guild).subject == Subject::Role(123));
}

#[tokio::test]
async fn unbound_sdk_directory_denies_without_a_request() {
    use super::super::{application::ChannelDirectory, domain::*};
    use super::inspection::DiscordDirectory;
    let directory = DiscordDirectory::default();
    assert_eq!(
        directory
            .member(
                ServerRef::parse("10").unwrap(),
                MemberRef::parse("11").unwrap()
            )
            .await
            .unwrap_err(),
        SpaceError::Permission
    );
}

#[test]
fn unknown_channel_response_is_distinct_from_lost_or_obfuscated_access() {
    use super::super::domain::SpaceError;
    use super::inspection::channel_status;
    assert_eq!(channel_status(404, 10003), SpaceError::ChannelMissing);
    for (status, code) in [(403, 50001), (404, 50001), (500, 10003)] {
        assert_eq!(channel_status(status, code), SpaceError::Permission);
    }
}

#[test]
fn production_create_payload_is_private_from_the_first_request() {
    use super::super::{application::ChannelCreation, domain::*};
    let input = ChannelCreation {
        guild: ServerRef::parse("10").unwrap(),
        category: ChannelRef::parse("13").unwrap(),
        owner: MemberRef::parse("11").unwrap(),
        bot: MemberRef::parse("99").unwrap(),
        generation: "12345678-1234-1234-1234-123456789abc".into(),
    };
    let request = to_value(create_definition(&input).unwrap()).unwrap();
    assert_eq!(request["type"], 0);
    assert_eq!(request["parent_id"], "13");
    assert_eq!(request["topic"], format!("fgspace:{}", input.generation));
    assert_eq!(
        request["permission_overwrites"].as_array().unwrap().len(),
        3
    );
    assert_eq!(
        request["permission_overwrites"][0]["deny"],
        Permissions::VIEW_CHANNEL.bits().to_string()
    );
}

#[tokio::test]
async fn shared_http_binding_is_one_time_and_preserves_client_identity() {
    use super::super::domain::SpaceError;
    use super::inspection::DiscordDirectory;
    use std::sync::Arc;
    let directory = DiscordDirectory::default();
    let http = Arc::new(serenity::http::Http::new("synthetic-unused-token"));
    directory.bind(http.clone()).unwrap();
    assert!(Arc::ptr_eq(directory.http().unwrap(), &http));
    assert_eq!(directory.bind(http), Err(SpaceError::Conflict));
}
