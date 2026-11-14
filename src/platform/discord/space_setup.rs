use super::diagnostics::SessionError;
use crate::{
    config::discord::SpaceConfig,
    features::{
        finance_spaces::{
            ChannelRef, MemberRef, ServerRef,
            application::{ChannelDirectory, LifecycleEvent, SpaceSettings, Spaces},
            infrastructure::{DiscordDirectory, PgSpaceStore, prepare_category},
            presentation::SpacesEvents,
        },
        identity::OwnerBinding,
    },
};
use serenity::http::Http;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use tokio::sync::{Notify, watch};
pub(super) struct Setup {
    pub(super) spaces: Spaces,
    pub(super) bot: Arc<AtomicU64>,
    directory: Arc<DiscordDirectory>,
    wake: Arc<Notify>,
}
pub(super) async fn build(
    config: Option<SpaceConfig>,
    guild: u64,
    pool: Option<sqlx::PgPool>,
    binding: Arc<OwnerBinding>,
    http: &Http,
) -> Result<Option<Setup>, SessionError> {
    let Some(config) = config else {
        return Ok(None);
    };
    let pool = pool.ok_or(SessionError::Setup)?;
    let guild = ServerRef::parse(&guild.to_string()).map_err(|_| SessionError::Setup)?;
    let configured = config
        .category
        .map(|id| ChannelRef::parse(&id.to_string()))
        .transpose()
        .map_err(|_| SessionError::Setup)?;
    let category = prepare_category(&pool, http, guild, configured)
        .await
        .map_err(|_| SessionError::Setup)?;
    let settings = SpaceSettings {
        guild,
        category,
        onboarding: config
            .onboarding
            .into_iter()
            .map(|id| ChannelRef::parse(&id.to_string()))
            .collect::<Result<_, _>>()
            .map_err(|_| SessionError::Setup)?,
    };
    let directory = Arc::new(DiscordDirectory::default());
    Ok(Some(Setup {
        spaces: Spaces::new(
            settings,
            Arc::new(PgSpaceStore::new(pool)),
            directory.clone(),
            binding,
        ),
        bot: Arc::new(AtomicU64::new(0)),
        directory,
        wake: Arc::new(Notify::new()),
    }))
}
impl Setup {
    pub(super) fn handler(&self, accepting: Arc<AtomicBool>) -> SpacesEvents {
        SpacesEvents {
            spaces: self.spaces.clone(),
            accepting,
            bot: self.bot.clone(),
            wake: self.wake.clone(),
        }
    }
    pub(super) async fn prepare(&self, http: Arc<Http>) -> Result<(), SessionError> {
        self.directory
            .bind(http.clone())
            .map_err(|_| SessionError::Setup)?;
        self.spaces
            .check_legacy()
            .await
            .map_err(|_| SessionError::Setup)?;
        let user = http
            .get_current_user()
            .await
            .map_err(|_| SessionError::Setup)?;
        let bot = MemberRef::parse(&user.id.to_string()).map_err(|_| SessionError::Setup)?;
        self.directory
            .setup(
                self.spaces.settings.guild,
                self.spaces.settings.category,
                self.spaces.settings.onboarding.clone(),
                bot,
            )
            .await
            .map_err(|_| SessionError::Setup)?;
        self.spaces
            .invalidate(LifecycleEvent::VerifyAll)
            .await
            .map_err(|_| SessionError::Setup)
    }
}
type Worker = Option<tokio::task::JoinHandle<()>>;
pub(super) fn start_worker(setup: Option<Setup>) -> (Option<watch::Sender<bool>>, Worker) {
    let Some(setup) = setup else {
        return (None, None);
    };
    let (stop, mut stopped) = watch::channel(false);
    let worker = tokio::spawn(async move {
        loop {
            tokio::select! {
                _=stopped.changed()=>return,
                _=setup.wake.notified()=>{}
            }
            if !setup.spaces.connected.load(Ordering::Acquire) {
                continue;
            }
            let Ok(bot) = MemberRef::parse(&setup.bot.load(Ordering::Acquire).to_string()) else {
                continue;
            };
            let mut after = 0;
            loop {
                let batch = tokio::select! {_=stopped.changed()=>return,result=setup.spaces.reconcile_batch(after,bot)=>result};
                match batch {
                    Ok(Some(next)) => after = next,
                    Ok(None) => break,
                    Err(_) => {
                        tracing::warn!(event = "space_reconciliation_failed");
                        break;
                    }
                }
            }
        }
    });
    (Some(stop), Some(worker))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn owned_worker_cancels_and_joins_without_provider_access() {
        let (spaces, _, _, bot) =
            crate::features::finance_spaces::application::ready_fixture().await;
        let wake = Arc::new(Notify::new());
        let setup = Setup {
            spaces,
            bot: Arc::new(AtomicU64::new(bot.get())),
            directory: Arc::new(DiscordDirectory::default()),
            wake: wake.clone(),
        };
        let (stop, worker) = start_worker(Some(setup));
        wake.notify_one();
        tokio::task::yield_now().await;
        stop.unwrap().send(true).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(1), worker.unwrap())
            .await
            .unwrap()
            .unwrap();
    }
}
