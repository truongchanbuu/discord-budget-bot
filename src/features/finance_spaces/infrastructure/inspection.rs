use super::super::{application::*, domain::*};
use chrono::{DateTime, Utc};
use serenity::{http::Http, model::prelude::*};
use std::sync::{Arc, OnceLock};

#[derive(Default)]
pub(crate) struct DiscordDirectory {
    http: OnceLock<Arc<Http>>,
}
impl DiscordDirectory {
    pub(crate) fn bind(&self, http: Arc<Http>) -> Result<(), SpaceError> {
        self.http.set(http).map_err(|_| SpaceError::Conflict)
    }
    pub(super) fn http(&self) -> Result<&Arc<Http>, SpaceError> {
        self.http.get().ok_or(SpaceError::Permission)
    }
}
pub(super) fn role_permissions(
    guild: &PartialGuild,
    member: &Member,
) -> Result<Permissions, SpaceError> {
    let mut permissions = guild
        .roles
        .get(&RoleId::new(guild.id.get()))
        .ok_or(SpaceError::Permission)?
        .permissions;
    for role in &member.roles {
        permissions |= guild
            .roles
            .get(role)
            .ok_or(SpaceError::Permission)?
            .permissions;
    }
    if member.user.id == guild.owner_id {
        permissions |= Permissions::ADMINISTRATOR;
    }
    Ok(permissions)
}
impl ChannelDirectory for DiscordDirectory {
    fn create(&self, input: ChannelCreation) -> SpaceFuture<'_, ChannelRef> {
        Box::pin(super::discord::create(self, input))
    }
    fn change_viewer(
        &self,
        channel: ChannelRef,
        member: MemberRef,
        access: DesiredAccess,
    ) -> SpaceFuture<'_, ()> {
        Box::pin(super::discord::change_viewer(self, channel, member, access))
    }
    fn candidates(
        &self,
        guild: ServerRef,
        category: ChannelRef,
        generation: String,
    ) -> SpaceFuture<'_, Vec<ChannelRef>> {
        Box::pin(super::discord::candidates(
            self, guild, category, generation,
        ))
    }
    fn setup(
        &self,
        guild: ServerRef,
        category: ChannelRef,
        onboarding: Vec<ChannelRef>,
        bot: MemberRef,
    ) -> SpaceFuture<'_, ()> {
        Box::pin(super::discord::setup(
            self, guild, category, onboarding, bot,
        ))
    }

    fn member(&self, guild: ServerRef, member: MemberRef) -> SpaceFuture<'_, MemberSnapshot> {
        Box::pin(async move {
            let http = self.http()?;
            let guild_id = GuildId::new(guild.get());
            let user = guild_id
                .member(http, UserId::new(member.get()))
                .await
                .map_err(|_| SpaceError::Permission)?;
            let guild = guild_id
                .to_partial_guild(http)
                .await
                .map_err(|_| SpaceError::Permission)?;
            let permissions = role_permissions(&guild, &user)?;
            let joined_at = user
                .joined_at
                .map(|at| {
                    DateTime::parse_from_rfc3339(&at.to_string())
                        .map(|at| at.with_timezone(&Utc))
                        .map_err(|_| SpaceError::Permission)
                })
                .transpose()?;
            Ok(MemberSnapshot {
                member,
                joined_at,
                human: !user.user.bot,
                pending: user.pending,
                elevated: permissions.intersects(
                    Permissions::ADMINISTRATOR
                        | Permissions::MANAGE_CHANNELS
                        | Permissions::MANAGE_ROLES
                        | Permissions::MANAGE_WEBHOOKS,
                ),
            })
        })
    }
    fn inspect(&self, channel: ChannelRef, bot: MemberRef) -> SpaceFuture<'_, ChannelSnapshot> {
        Box::pin(async move {
            let http = self.http()?;
            let channel = ChannelId::new(channel.get())
                .to_channel(http)
                .await
                .map_err(channel_error)?
                .guild()
                .ok_or(SpaceError::Permission)?;
            let guild = channel
                .guild_id
                .to_partial_guild(http)
                .await
                .map_err(|_| SpaceError::Permission)?;
            let member = channel
                .guild_id
                .member(http, UserId::new(bot.get()))
                .await
                .map_err(|_| SpaceError::Permission)?;
            let permissions = role_permissions(&guild, &member)?;
            let required = Permissions::MANAGE_CHANNELS
                | Permissions::MANAGE_ROLES
                | Permissions::VIEW_CHANNEL
                | Permissions::SEND_MESSAGES
                | Permissions::READ_MESSAGE_HISTORY
                | Permissions::CREATE_PUBLIC_THREADS
                | Permissions::CREATE_PRIVATE_THREADS
                | Permissions::SEND_MESSAGES_IN_THREADS;
            let overwrites = channel
                .permission_overwrites
                .iter()
                .map(|entry| normalize_overwrite(entry, guild.id))
                .collect();
            Ok(ChannelSnapshot {
                channel: ChannelRef::parse(&channel.id.to_string())?,
                guild: ServerRef::parse(&guild.id.to_string())?,
                parent: channel
                    .parent_id
                    .map(|id| ChannelRef::parse(&id.to_string()))
                    .transpose()?,
                text: channel.kind == ChannelType::Text,
                marker: channel.topic,
                bot_capable: permissions.contains(required)
                    && !permissions.contains(Permissions::ADMINISTRATOR),
                overwrites,
                observed_at: Utc::now(),
            })
        })
    }
}
pub(super) fn normalize_overwrite(entry: &PermissionOverwrite, guild: GuildId) -> Overwrite {
    let subject = match entry.kind {
        PermissionOverwriteType::Role(id) if id.get() == guild.get() => Subject::Everyone,
        PermissionOverwriteType::Role(id) => Subject::Role(id.get()),
        PermissionOverwriteType::Member(id) => match MemberRef::parse(&id.to_string()) {
            Ok(id) => Subject::Member(id),
            Err(_) => {
                return Overwrite {
                    subject: Subject::Role(0),
                    profile: AccessProfile::Other,
                };
            }
        },
        _ => {
            return Overwrite {
                subject: Subject::Role(0),
                profile: AccessProfile::Other,
            };
        }
    };
    let writable =
        Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::READ_MESSAGE_HISTORY;
    let readonly = Permissions::VIEW_CHANNEL | Permissions::READ_MESSAGE_HISTORY;
    let posting = Permissions::SEND_MESSAGES
        | Permissions::CREATE_PUBLIC_THREADS
        | Permissions::CREATE_PRIVATE_THREADS
        | Permissions::SEND_MESSAGES_IN_THREADS;
    let profile = if entry.allow.is_empty() && entry.deny == Permissions::VIEW_CHANNEL {
        AccessProfile::Hidden
    } else if entry.allow == writable && entry.deny.is_empty() {
        AccessProfile::Writable
    } else if entry.allow == readonly && entry.deny == posting {
        AccessProfile::Viewer
    } else {
        AccessProfile::Other
    };
    Overwrite { subject, profile }
}

fn channel_error(error: serenity::Error) -> SpaceError {
    if let serenity::Error::Http(serenity::http::HttpError::UnsuccessfulRequest(response)) = error {
        return channel_status(response.status_code.as_u16(), response.error.code);
    }
    SpaceError::Permission
}
pub(super) fn channel_status(status: u16, code: isize) -> SpaceError {
    if status == 404 && code == 10003 {
        SpaceError::ChannelMissing
    } else {
        SpaceError::Permission
    }
}
