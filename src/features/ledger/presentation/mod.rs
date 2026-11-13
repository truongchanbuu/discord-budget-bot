mod components;
mod messages;
mod modals;
mod views;
use super::{Journal, application::Scope, domain::JournalError};
use crate::features::identity::{DiscordAccountId, OwnerBinding, TrustedActor, resolve_owner};
use poise::serenity_prelude as serenity;
use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub(crate) struct FinanceEvents {
    pub journal: Journal,
    pub binding: Arc<OwnerBinding>,
    pub accepting: Arc<AtomicBool>,
}
impl FinanceEvents {
    fn scope<'a>(&'a self, actor: &'a TrustedActor, channel: &'a str) -> Scope<'a> {
        Scope {
            actor,
            binding: &self.binding,
            channel,
        }
    }
}

fn actor(
    user: &serenity::User,
    guild: Option<serenity::GuildId>,
    binding: &OwnerBinding,
) -> Result<TrustedActor, JournalError> {
    if guild.is_some() || user.bot || user.name.is_empty() {
        return Err(JournalError::Denied);
    }
    let actor = TrustedActor::new(
        DiscordAccountId::parse(&user.id.to_string()).map_err(|_| JournalError::Denied)?,
    );
    resolve_owner(Some(&actor), Some(binding)).map_err(|_| JournalError::Denied)?;
    Ok(actor)
}

#[serenity::async_trait]
impl serenity::EventHandler for FinanceEvents {
    async fn message(&self, context: serenity::Context, message: serenity::Message) {
        if message.kind != serenity::MessageType::Regular
            || !message.attachments.is_empty()
            || !message.sticker_items.is_empty()
            || message.guild_id.is_some()
            || message.author.bot
            || !self.accepting.load(Ordering::Acquire)
        {
            return;
        }
        if messages::handle(self, &context, &message).await.is_err() {
            tracing::warn!(event = "finance_response_failed");
        }
    }
    async fn interaction_create(
        &self,
        context: serenity::Context,
        interaction: serenity::Interaction,
    ) {
        let result = match interaction {
            serenity::Interaction::Component(value)
                if value.data.custom_id.starts_with("fgfinance:") =>
            {
                components::button(self, &context, &value).await
            }
            serenity::Interaction::Modal(value)
                if value.data.custom_id.starts_with("fgfinance:") =>
            {
                components::modal(self, &context, &value).await
            }
            _ => return,
        };
        if result.is_err() {
            tracing::warn!(event = "finance_response_failed");
        }
    }
}

async fn send<T>(
    future: impl Future<Output = Result<T, serenity::Error>>,
) -> Result<T, JournalError> {
    tokio::time::timeout(Duration::from_secs(2), future)
        .await
        .map_err(|_| JournalError::Timeout)?
        .map_err(|_| JournalError::Delivery)
}
struct Reference<'a> {
    action: &'a str,
    id: i64,
    revision: i32,
}
fn reference(action: &str, id: i64, revision: i32) -> String {
    format!("fgfinance:{action}:{id}:{revision}")
}
fn parse_reference(value: &str) -> Result<Reference<'_>, JournalError> {
    let parts: Vec<_> = value.split(':').collect();
    if value.len() > 100
        || parts.len() != 4
        || parts[0] != "fgfinance"
        || !["save", "edit", "cancel", "modal"].contains(&parts[1])
    {
        return Err(JournalError::InvalidInput);
    }
    let id = parts[2]
        .parse::<i64>()
        .map_err(|_| JournalError::InvalidInput)?;
    let revision = parts[3]
        .parse::<i32>()
        .map_err(|_| JournalError::InvalidInput)?;
    if id <= 0 || revision < 0 {
        return Err(JournalError::InvalidInput);
    }
    Ok(Reference {
        action: parts[1],
        id,
        revision,
    })
}
#[cfg(test)]
mod tests;

async fn acknowledged_action<T>(
    acknowledgement: impl Future<Output = Result<(), JournalError>>,
    action: impl Future<Output = Result<T, JournalError>>,
) -> Result<Result<T, JournalError>, JournalError> {
    // REMIND: Do not poll a financial action if private acknowledgement failed or was uncertain.
    acknowledgement.await?;
    Ok(action.await)
}
