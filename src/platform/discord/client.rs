use super::{
    diagnostics::{self, SessionError},
    lifecycle, registration,
};
use crate::{
    config::discord::DiscordConfig,
    features::identity::presentation::{CommandData, ReadinessError, fundguard},
    features::ledger::{infrastructure::PgStore, presentation::FinanceEvents},
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

pub(crate) async fn run(
    config: DiscordConfig,
    pool: Option<sqlx::PgPool>,
) -> Result<(), SessionError> {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut interrupt = Box::pin(tokio::signal::ctrl_c());
    let guild = serenity::GuildId::new(config.guild.get());
    let mut data = CommandData::new(config.binding, guild);
    if pool.is_some() {
        data = data.with_finance_dm();
    }
    let binding = data.binding.clone();
    let accepting = data.accepting.clone();
    let (sender, setup) = oneshot::channel();
    let http = serenity::http::Http::new(config.token.as_str());
    let configured_spaces = lifecycle::prepare(
        super::space_setup::build(
            config.spaces,
            config.guild.get(),
            pool.clone(),
            binding.clone(),
            &http,
        ),
        interrupt.as_mut(),
        deadline,
    )
    .await;
    let space_setup = match configured_spaces {
        Ok(Some(setup)) => setup,
        result => {
            if let Some(pool) = &pool {
                pool.close().await;
            }
            return result.map(|_| ());
        }
    };
    let intents = if space_setup.is_some() {
        INTENTS
            | serenity::GatewayIntents::GUILDS
            | serenity::GatewayIntents::GUILD_MESSAGES
            | serenity::GatewayIntents::GUILD_MEMBERS
            | serenity::GatewayIntents::DIRECT_MESSAGES
    } else if pool.is_some() {
        INTENTS | serenity::GatewayIntents::DIRECT_MESSAGES
    } else {
        INTENTS
    };
    let mut builder = serenity::ClientBuilder::new_with_http(http, intents)
        .framework(framework(data, guild, deadline, sender));
    if let Some(database) = &pool {
        builder = builder.event_handler(FinanceEvents {
            journal: match &space_setup {
                Some(setup) => PgStore::journal(database.clone()).with_spaces(setup.spaces.clone()),
                None => PgStore::journal(database.clone()),
            },
            spaces: space_setup.as_ref().map(|s| s.spaces.clone()),
            bot: space_setup
                .as_ref()
                .map(|s| s.bot.clone())
                .unwrap_or_default(),
            binding,
            accepting: accepting.clone(),
        });
    }
    if let Some(setup) = &space_setup {
        builder = builder.event_handler(setup.handler(accepting.clone()));
    }
    let prepared =
        lifecycle::prepare(async move { builder.await }, interrupt.as_mut(), deadline).await;
    let mut client = match prepared {
        Ok(Some(client)) => client,
        result => {
            if let Some(pool) = pool {
                tokio::time::timeout(Duration::from_secs(2), pool.close())
                    .await
                    .map_err(|_| SessionError::ShutdownTimeout)?;
            }
            return result.map(|_| ());
        }
    };
    if let Some(setup) = &space_setup
        && !matches!(
            tokio::time::timeout_at(deadline, setup.prepare(client.http.clone())).await,
            Ok(Ok(()))
        )
    {
        if let Some(pool) = &pool {
            pool.close().await;
        }
        return Err(SessionError::Setup);
    }
    let (space_stop, space_worker) = super::space_setup::start_worker(space_setup);
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
            if let Some(stop) = space_stop {
                let _ = stop.send(true);
            }
            if let Some(worker) = space_worker {
                worker.await.map_err(|_| SessionError::Task)?;
            }
            manager.shutdown_all().await;
            if let Some(pool) = pool {
                pool.close().await;
            }
            Ok(())
        },
    )
    .await
}

#[cfg(test)]
#[path = "client_tests.rs"]
mod tests;
