mod components;
mod messages;
mod references;
#[cfg(test)]
mod tests;
mod views;
use super::{application::*, domain::*};
use serenity::{async_trait, model::prelude::*, prelude::*};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

pub(crate) struct SpacesEvents {
    pub(crate) spaces: Spaces,
    pub(crate) accepting: Arc<AtomicBool>,
    pub(crate) bot: Arc<AtomicU64>,
    pub(crate) wake: Arc<tokio::sync::Notify>,
}
#[async_trait]
impl EventHandler for SpacesEvents {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if !self.accepting.load(Ordering::Acquire) {
            return;
        }
        if let Interaction::Component(value) = interaction
            && value.data.custom_id.starts_with("fgspace:")
            && components::handle(self, &context, &value).await.is_err()
        {
            tracing::warn!(event = "space_control_failed");
        }
    }
    async fn shard_stage_update(
        &self,
        _context: Context,
        event: serenity::gateway::ShardStageUpdateEvent,
    ) {
        self.gateway_stage(event.new).await;
    }
    async fn ready(&self, _context: Context, ready: Ready) {
        self.spaces.connected.store(false, Ordering::Release);
        if self
            .spaces
            .invalidate(LifecycleEvent::VerifyAll)
            .await
            .is_ok()
        {
            self.bot.store(ready.user.id.get(), Ordering::Release);
            self.spaces.connected.store(true, Ordering::Release);
            self.wake.notify_one();
        }
    }
    async fn resume(&self, _context: Context, _event: ResumedEvent) {
        self.spaces.connected.store(false, Ordering::Release);
        if self
            .spaces
            .invalidate(LifecycleEvent::VerifyAll)
            .await
            .is_ok()
        {
            self.spaces.connected.store(true, Ordering::Release);
            self.wake.notify_one();
        }
    }
    async fn guild_member_removal(
        &self,
        _context: Context,
        guild: GuildId,
        user: User,
        _old: Option<Member>,
    ) {
        if guild.get() == self.spaces.settings.guild.get()
            && let Ok(member) = MemberRef::parse(&user.id.to_string())
        {
            self.invalidate(LifecycleEvent::MemberRemoved(member)).await;
        }
    }
    async fn guild_member_addition(&self, _context: Context, member: Member) {
        if member.guild_id.get() == self.spaces.settings.guild.get()
            && let Ok(id) = MemberRef::parse(&member.user.id.to_string())
        {
            self.invalidate(LifecycleEvent::MemberChanged(id)).await;
        }
    }
    async fn guild_member_update(
        &self,
        _context: Context,
        _old: Option<Member>,
        _new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        if event.guild_id.get() == self.spaces.settings.guild.get() {
            self.invalidate(LifecycleEvent::VerifyAll).await;
        }
    }
    async fn guild_role_update(&self, _context: Context, _old: Option<Role>, role: Role) {
        if role.guild_id.get() == self.spaces.settings.guild.get() {
            self.invalidate(LifecycleEvent::VerifyAll).await;
        }
    }
    async fn guild_role_delete(
        &self,
        _context: Context,
        guild: GuildId,
        _id: RoleId,
        _old: Option<Role>,
    ) {
        if guild.get() == self.spaces.settings.guild.get() {
            self.invalidate(LifecycleEvent::VerifyAll).await;
        }
    }
    async fn channel_update(
        &self,
        _context: Context,
        _old: Option<GuildChannel>,
        new: GuildChannel,
    ) {
        if new.guild_id.get() == self.spaces.settings.guild.get()
            && let Ok(id) = ChannelRef::parse(&new.id.to_string())
        {
            self.invalidate(LifecycleEvent::ChannelChanged(id)).await;
        }
    }
    async fn channel_delete(
        &self,
        _context: Context,
        channel: GuildChannel,
        _messages: Option<Vec<Message>>,
    ) {
        if channel.guild_id.get() == self.spaces.settings.guild.get()
            && let Ok(id) = ChannelRef::parse(&channel.id.to_string())
        {
            self.invalidate(LifecycleEvent::ChannelDeleted(id)).await;
        }
    }
    async fn message(&self, context: Context, message: Message) {
        if !self.accepting.load(Ordering::Acquire)
            || !self.spaces.connected.load(Ordering::Acquire)
            || message.guild_id.map(|id| id.get()) != Some(self.spaces.settings.guild.get())
        {
            return;
        }
        let Ok(bot) = MemberRef::parse(&self.bot.load(Ordering::Acquire).to_string()) else {
            return;
        };
        if messages::handle(self, &context, &message, bot)
            .await
            .is_err()
        {
            tracing::warn!(event = "space_response_failed");
        }
    }
}

impl SpacesEvents {
    async fn gateway_stage(&self, stage: serenity::gateway::ConnectionStage) {
        if stage != serenity::gateway::ConnectionStage::Connected {
            self.spaces.connected.store(false, Ordering::Release);
            self.invalidate(LifecycleEvent::VerifyAll).await;
        }
    }
    async fn invalidate(&self, event: LifecycleEvent) {
        if self.spaces.invalidate(event).await.is_err() {
            self.spaces.connected.store(false, Ordering::Release);
            tracing::warn!(event = "space_invalidation_failed");
        } else {
            self.wake.notify_one();
        }
    }
}

#[cfg(test)]
mod lifecycle_tests;

#[cfg(test)]
mod access_tests;
