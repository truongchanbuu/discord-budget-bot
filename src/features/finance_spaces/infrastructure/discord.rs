use serenity::model::prelude::*;

pub(super) fn canonical_overwrites(
    guild: GuildId,
    owner: UserId,
    bot: UserId,
    viewers: &[UserId],
) -> Vec<PermissionOverwrite> {
    let writable =
        Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY;
    let mut result = vec![
        PermissionOverwrite {
            kind: PermissionOverwriteType::Role(RoleId::new(guild.get())),
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
        },
        PermissionOverwrite {
            kind: PermissionOverwriteType::Member(owner),
            allow: writable,
            deny: Permissions::empty(),
        },
        PermissionOverwrite {
            kind: PermissionOverwriteType::Member(bot),
            allow: writable,
            deny: Permissions::empty(),
        },
    ];
    for viewer in viewers {
        result.push(PermissionOverwrite {
            kind: PermissionOverwriteType::Member(*viewer),
            allow: Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY,
            deny: Permissions::SEND_MESSAGES
                | Permissions::CREATE_PUBLIC_THREADS
                | Permissions::CREATE_PRIVATE_THREADS
                | Permissions::SEND_MESSAGES_IN_THREADS,
        });
    }
    result
}

#[cfg(test)]
pub(super) fn mentioned_input(message: &Message, bot: UserId) -> Option<&str> {
    if message.author.bot
        || message.webhook_id.is_some()
        || message.kind != MessageType::Regular
        || !message.attachments.is_empty()
        || !message.sticker_items.is_empty()
        || !message.mentions.iter().any(|user| user.id == bot)
    {
        return None;
    }
    for prefix in [format!("<@{}>", bot.get()), format!("<@!{}>", bot.get())] {
        if let Some(rest) = message.content.strip_prefix(&prefix)
            && (rest.is_empty() || rest.starts_with(char::is_whitespace))
        {
            return Some(rest.trim());
        }
    }
    None
}

use super::super::{application::ChannelCreation, domain::*};
use super::inspection::{DiscordDirectory, role_permissions};
use serenity::builder::CreateChannel;

pub(super) async fn create(
    directory: &DiscordDirectory,
    input: ChannelCreation,
) -> Result<ChannelRef, SpaceError> {
    let builder = create_definition(&input)?;
    setup(directory, input.guild, input.category, vec![], input.bot).await?;
    let channels = GuildId::new(input.guild.get())
        .channels(directory.http()?)
        .await
        .map_err(|_| SpaceError::Permission)?;
    if channels
        .values()
        .filter(|channel| channel.parent_id == Some(ChannelId::new(input.category.get())))
        .count()
        >= 50
    {
        return Err(SpaceError::Capacity);
    }

    let channel = GuildId::new(input.guild.get())
        .create_channel(directory.http()?, builder)
        .await
        .map_err(|_| SpaceError::Uncertain)?;
    ChannelRef::parse(&channel.id.to_string())
}
pub(super) async fn change_viewer(
    directory: &DiscordDirectory,
    channel: ChannelRef,
    member: MemberRef,
    access: DesiredAccess,
) -> Result<(), SpaceError> {
    let id = ChannelId::new(channel.get());
    let user = UserId::new(member.get());
    match access {
        DesiredAccess::Viewer => {
            let overwrite = PermissionOverwrite {
                kind: PermissionOverwriteType::Member(user),
                allow: Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY,
                deny: Permissions::SEND_MESSAGES
                    | Permissions::CREATE_PUBLIC_THREADS
                    | Permissions::CREATE_PRIVATE_THREADS
                    | Permissions::SEND_MESSAGES_IN_THREADS,
            };
            id.create_permission(directory.http()?, overwrite)
                .await
                .map_err(|_| SpaceError::Uncertain)?;
        }
        DesiredAccess::Absent => id
            .delete_permission(directory.http()?, PermissionOverwriteType::Member(user))
            .await
            .map_err(|_| SpaceError::Uncertain)?,
    }
    Ok(())
}
pub(super) async fn candidates(
    directory: &DiscordDirectory,
    guild: ServerRef,
    category: ChannelRef,
    generation: String,
) -> Result<Vec<ChannelRef>, SpaceError> {
    if !valid_token(&generation) {
        return Err(SpaceError::Invalid);
    }
    let channels = GuildId::new(guild.get())
        .channels(directory.http()?)
        .await
        .map_err(|_| SpaceError::Permission)?;
    channels
        .values()
        .filter(|c| {
            c.parent_id == Some(ChannelId::new(category.get()))
                && c.topic.as_deref() == Some(format!("fgspace:{generation}").as_str())
        })
        .map(|c| ChannelRef::parse(&c.id.to_string()))
        .collect()
}
pub(super) async fn setup(
    directory: &DiscordDirectory,
    guild: ServerRef,
    category: ChannelRef,
    onboarding: Vec<ChannelRef>,
    bot: MemberRef,
) -> Result<(), SpaceError> {
    let http = directory.http()?;
    let guild_id = GuildId::new(guild.get());
    let parent = ChannelId::new(category.get())
        .to_channel(http)
        .await
        .map_err(|_| SpaceError::Permission)?
        .guild()
        .ok_or(SpaceError::Permission)?;
    if parent.guild_id != guild_id || parent.kind != ChannelType::Category {
        return Err(SpaceError::Permission);
    }
    for id in onboarding {
        let channel = ChannelId::new(id.get())
            .to_channel(http)
            .await
            .map_err(|_| SpaceError::Permission)?
            .guild()
            .ok_or(SpaceError::Permission)?;
        if channel.guild_id != guild_id || channel.kind != ChannelType::Text {
            return Err(SpaceError::Permission);
        }
    }
    let guild = guild_id
        .to_partial_guild(http)
        .await
        .map_err(|_| SpaceError::Permission)?;
    let member = guild_id
        .member(http, UserId::new(bot.get()))
        .await
        .map_err(|_| SpaceError::Permission)?;
    let permissions = role_permissions(&guild, &member)?;
    verify_bot_permissions(permissions)
}

pub(super) fn verify_bot_permissions(permissions: Permissions) -> Result<(), SpaceError> {
    let required = Permissions::MANAGE_CHANNELS
        | Permissions::MANAGE_ROLES
        | Permissions::VIEW_CHANNEL
        | Permissions::SEND_MESSAGES
        | Permissions::READ_MESSAGE_HISTORY
        | Permissions::CREATE_PUBLIC_THREADS
        | Permissions::CREATE_PRIVATE_THREADS
        | Permissions::SEND_MESSAGES_IN_THREADS;
    if !permissions.contains(required) || permissions.contains(Permissions::ADMINISTRATOR) {
        return Err(SpaceError::Permission);
    }
    Ok(())
}

pub(super) fn create_definition(
    input: &ChannelCreation,
) -> Result<CreateChannel<'static>, SpaceError> {
    if !valid_token(&input.generation) {
        return Err(SpaceError::Invalid);
    }
    Ok(CreateChannel::new("tai-chinh")
        .kind(ChannelType::Text)
        .category(ChannelId::new(input.category.get()))
        .topic(format!("fgspace:{}", input.generation))
        .audit_log_reason("FundGuard private space creation")
        .permissions(canonical_overwrites(
            GuildId::new(input.guild.get()),
            UserId::new(input.owner.get()),
            UserId::new(input.bot.get()),
            &[],
        )))
}
