use super::{
    diagnostics::{self, SessionError},
    lifecycle, registration,
};
use crate::{
    config::discord::DiscordConfig,
    features::identity::presentation::{CommandData, ReadinessError, fundguard},
};
use poise::serenity_prelude as serenity;
use std::{future::Future, time::Duration};
use tokio::{sync::oneshot, time::Instant};

const INTENTS: serenity::GatewayIntents = serenity::GatewayIntents::empty();

fn framework(
    data: CommandData,
    guild: serenity::GuildId,
    deadline: Instant,
    sender: oneshot::Sender<Result<(), SessionError>>,
) -> poise::Framework<CommandData, ReadinessError> {
    let options = poise::FrameworkOptions {
        commands: vec![fundguard()],
        skip_checks_for_owners: false,
        on_error: |error| Box::pin(diagnostics::on_error(error)),
        ..Default::default()
    };
    poise::Framework::builder()
        .initialize_owners(false)
        .options(options)
        .setup(move |context, _, _| {
            Box::pin(async move {
                complete_setup(
                    data,
                    sender,
                    registration::register(
                        guild,
                        fundguard(),
                        deadline,
                        |guild, definition| async move {
                            guild
                                .create_command(&context.http, definition)
                                .await
                                .map(|_| ())
                        },
                    ),
                )
                .await
            })
        })
        .build()
}

async fn complete_setup(
    data: CommandData,
    sender: oneshot::Sender<Result<(), SessionError>>,
    register: impl Future<Output = Result<(), SessionError>>,
) -> Result<CommandData, ReadinessError> {
    let result = register.await;
    let _ = sender.send(result);
    result.map(|()| data).map_err(|_| ReadinessError)
}

pub(crate) async fn run(config: DiscordConfig) -> Result<(), SessionError> {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut interrupt = Box::pin(tokio::signal::ctrl_c());
    let guild = serenity::GuildId::new(config.guild.get());
    let data = CommandData::new(config.binding, guild);
    let accepting = data.accepting.clone();
    let (sender, setup) = oneshot::channel();
    let builder = serenity::Client::builder(config.token.as_str(), INTENTS)
        .framework(framework(data, guild, deadline, sender));
    let Some(mut client) =
        lifecycle::prepare(async move { builder.await }, interrupt.as_mut(), deadline).await?
    else {
        return Ok(());
    };
    let manager = client.shard_manager.clone();
    let status = manager.clone();
    let task =
        tokio::spawn(async move { client.start().await.map_err(|_| SessionError::Connection) });
    lifecycle::supervise(
        task,
        setup,
        interrupt,
        accepting,
        deadline,
        move || {
            let manager = status.clone();
            async move {
                let runners = manager.runners.lock().await;
                runners.len() == 1
                    && runners
                        .values()
                        .all(|runner| runner.stage == serenity::ConnectionStage::Connected)
            }
        },
        move || async move {
            manager.shutdown_all().await;
            Ok(())
        },
    )
    .await
}

#[cfg(test)]
#[path = "client_tests.rs"]
mod tests;
