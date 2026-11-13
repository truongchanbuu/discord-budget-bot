use super::diagnostics::SessionError;
use crate::features::identity::presentation::{CommandData, ReadinessError};
use poise::serenity_prelude as serenity;
use std::{future::Future, time::Duration};
use tokio::time::{Instant, timeout_at};

pub(super) async fn register<F, E>(
    guild: serenity::GuildId,
    command: poise::Command<CommandData, ReadinessError>,
    startup_deadline: Instant,
    upsert: impl FnOnce(serenity::GuildId, serenity::CreateCommand) -> F,
) -> Result<(), SessionError>
where
    F: Future<Output = Result<(), E>>,
{
    let definition = command
        .create_as_slash_command()
        .ok_or(SessionError::Registration)?
        .kind(serenity::CommandType::ChatInput);
    let deadline = startup_deadline.min(Instant::now() + Duration::from_secs(10));
    match timeout_at(deadline, upsert(guild, definition)).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(_)) => Err(SessionError::Registration),
        Err(_) => Err(SessionError::RegistrationTimeout),
    }
}

#[cfg(test)]
#[path = "registration_tests.rs"]
mod tests;
