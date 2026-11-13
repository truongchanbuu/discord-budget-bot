use super::diagnostics::SessionError;
use std::{
    future::{Future, pending},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    sync::oneshot,
    task::JoinHandle,
    time::{Instant, interval, sleep_until, timeout_at},
};

pub(super) async fn prepare<T, E>(
    build: impl Future<Output = Result<T, E>>,
    interrupt: Pin<&mut impl Future<Output = Result<(), std::io::Error>>>,
    deadline: Instant,
) -> Result<Option<T>, SessionError> {
    tokio::select! {
        biased;
        signal = interrupt => signal.map(|()| None).map_err(|_| SessionError::Signal),
        result = timeout_at(deadline, build) => match result {
            Ok(Ok(client)) => Ok(Some(client)),
            Ok(Err(_)) => Err(SessionError::Build),
            Err(_) => Err(SessionError::StartupTimeout),
        }
    }
}

pub(super) async fn supervise<I, S, SF, C, CF>(
    mut client: JoinHandle<Result<(), SessionError>>,
    mut setup: oneshot::Receiver<Result<(), SessionError>>,
    interrupt: I,
    accepting: Arc<AtomicBool>,
    startup_deadline: Instant,
    mut sample: S,
    close: C,
) -> Result<(), SessionError>
where
    I: Future<Output = Result<(), std::io::Error>>,
    S: FnMut() -> SF,
    SF: Future<Output = bool>,
    C: FnOnce() -> CF,
    CF: Future<Output = Result<(), SessionError>>,
{
    tokio::pin!(interrupt);
    let mut ready = false;
    let mut recovery = None;
    let mut ticks = interval(Duration::from_secs(1));
    let mut consumed = false;
    let outcome = loop {
        let deadline = if ready {
            recovery
        } else {
            Some(startup_deadline)
        };
        tokio::select! {
            biased;
            () = async {
                match deadline {
                    Some(deadline) => sleep_until(deadline).await,
                    None => pending().await,
                }
            } => break Err(if ready {
                SessionError::RecoveryTimeout
            } else {
                SessionError::StartupTimeout
            }),
            signal = &mut interrupt => break signal.map_err(|_| SessionError::Signal),
            result = &mut client => {
                consumed = true;
                break match result {
                    Ok(Err(error)) => Err(error),
                    Ok(Ok(())) => Err(SessionError::Connection),
                    Err(_) => Err(SessionError::Task),
                };
            }
            result = &mut setup, if !ready => match result {
                Ok(Ok(())) => {
                    ready = true;
                    tracing::info!(event = "discord_ready");
                }
                Ok(Err(error)) => break Err(error),
                Err(_) => break Err(SessionError::Setup),
            },
            connected = async {
                ticks.tick().await;
                sample().await
            }, if ready => {
                if connected {
                    recovery = None;
                } else if recovery.is_none() {
                    recovery = Some(Instant::now() + Duration::from_secs(30));
                }
            }
        }
    };
    accepting.store(false, Ordering::Release);
    tracing::info!(event = "discord_stopping");
    cleanup(&mut client, consumed, close()).await?;
    outcome
}

async fn cleanup(
    client: &mut JoinHandle<Result<(), SessionError>>,
    consumed: bool,
    close: impl Future<Output = Result<(), SessionError>>,
) -> Result<(), SessionError> {
    let deadline = Instant::now() + Duration::from_secs(10);
    let closed = timeout_at(deadline.min(Instant::now() + Duration::from_secs(8)), close).await;
    if !consumed {
        client.abort();
        if timeout_at(deadline, client).await.is_err() {
            return Err(SessionError::ShutdownTimeout);
        }
    }
    closed
        .map_err(|_| SessionError::ShutdownTimeout)?
        .map_err(|_| SessionError::Shutdown)?;
    tracing::info!(event = "discord_stopped");
    Ok(())
}

#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "shutdown_tests.rs"]
mod shutdown_tests;
