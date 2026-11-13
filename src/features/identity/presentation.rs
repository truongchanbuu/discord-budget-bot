use super::{DiscordAccountId, OwnerBinding, TrustedActor, resolve_owner};
use poise::serenity_prelude as serenity;
use std::{
    fmt,
    future::Future,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

pub(crate) struct CommandData {
    binding: OwnerBinding,
    guild: serenity::GuildId,
    pub(crate) accepting: Arc<AtomicBool>,
}

impl CommandData {
    pub(crate) fn new(binding: OwnerBinding, guild: serenity::GuildId) -> Self {
        Self {
            binding,
            guild,
            accepting: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl fmt::Debug for CommandData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CommandData([redacted])")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ReadinessDecision {
    Ready,
    Denied,
}

#[derive(Debug, thiserror::Error)]
#[error("private response unavailable")]
pub(crate) struct ReadinessError;

fn decide(data: &CommandData, interaction: &serenity::CommandInteraction) -> ReadinessDecision {
    // REMIND: Missing SDK user objects have default fields; reject them before identity mapping.
    if !data.accepting.load(Ordering::Acquire)
        || interaction.guild_id != Some(data.guild)
        || interaction.user.name.is_empty()
    {
        return ReadinessDecision::Denied;
    }
    let actor = DiscordAccountId::parse(&interaction.user.id.to_string())
        .ok()
        .map(TrustedActor::new);
    if resolve_owner(actor.as_ref(), Some(&data.binding)).is_ok() {
        ReadinessDecision::Ready
    } else {
        ReadinessDecision::Denied
    }
}

fn reply(decision: ReadinessDecision) -> poise::CreateReply {
    let content = match decision {
        ReadinessDecision::Ready => "FundGuard is ready. No financial operations are available.",
        ReadinessDecision::Denied => "Access denied.",
    };
    poise::CreateReply::default()
        .content(content)
        .ephemeral(true)
        .allowed_mentions(serenity::CreateAllowedMentions::new())
}

async fn send_private<F, E>(
    decision: ReadinessDecision,
    send: impl FnOnce(poise::CreateReply) -> F,
) -> Result<(), ReadinessError>
where
    F: Future<Output = Result<(), E>>,
{
    match tokio::time::timeout(Duration::from_secs(2), send(reply(decision))).await {
        Ok(Ok(())) => Ok(()),
        _ => Err(ReadinessError),
    }
}

#[poise::command(slash_command, ephemeral)]
/// Check FundGuard readiness.
pub(crate) async fn fundguard(
    context: poise::Context<'_, CommandData, ReadinessError>,
) -> Result<(), ReadinessError> {
    let poise::Context::Application(application) = context else {
        return Err(ReadinessError);
    };
    if !context.data().accepting.load(Ordering::Acquire) {
        return Err(ReadinessError);
    }
    let decision = decide(context.data(), application.interaction);
    if decision == ReadinessDecision::Denied {
        tracing::info!(event = "discord_request_denied");
    }
    send_private(decision, |response| async move {
        context.send(response).await.map(|_| ())
    })
    .await
}

#[cfg(test)]
mod tests;
