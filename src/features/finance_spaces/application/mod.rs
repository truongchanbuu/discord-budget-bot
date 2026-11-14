mod ports;
pub(crate) use ports::*;

#[cfg(test)]
mod test_support;

mod authorization;

#[cfg(test)]
mod provisioning_tests;

mod creation_contract;
pub(crate) use creation_contract::*;

use super::domain::*;
use crate::features::identity::OwnerBinding;
use std::{sync::Arc, time::Duration};

pub(crate) struct SpaceSettings {
    pub(crate) guild: ServerRef,
    pub(crate) category: ChannelRef,
    pub(crate) onboarding: Vec<ChannelRef>,
}
impl SpaceSettings {
    pub(crate) fn permits_onboarding(&self, channel: ChannelRef) -> bool {
        self.onboarding.is_empty() || self.onboarding.contains(&channel)
    }
}
#[derive(Clone)]
pub(crate) struct Spaces {
    pub(crate) settings: Arc<SpaceSettings>,
    pub(super) store: Arc<dyn SpaceStore>,
    pub(super) directory: Arc<dyn ChannelDirectory>,
    pub(super) legacy: Arc<OwnerBinding>,
    pub(crate) connected: Arc<std::sync::atomic::AtomicBool>,
    limiter: Arc<std::sync::Mutex<rate_limit::CommandLimiter>>,
}
impl Spaces {
    pub(crate) fn new(
        settings: SpaceSettings,
        store: Arc<dyn SpaceStore>,
        directory: Arc<dyn ChannelDirectory>,
        legacy: Arc<OwnerBinding>,
    ) -> Self {
        Self {
            settings: Arc::new(settings),
            store,
            directory,
            legacy,
            connected: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            limiter: Arc::new(std::sync::Mutex::new(rate_limit::CommandLimiter::default())),
        }
    }
}
async fn bounded<T>(
    future: impl std::future::Future<Output = Result<T, SpaceError>>,
) -> Result<T, SpaceError> {
    tokio::time::timeout(Duration::from_secs(5), future)
        .await
        .map_err(|_| SpaceError::Timeout)?
}
mod provisioning;

mod routing;
pub(crate) use routing::*;

#[cfg(test)]
mod recovery_tests;

mod lifecycle_contract;
pub(crate) use lifecycle_contract::*;
mod recovery;

#[cfg(test)]
mod authorization_tests;

pub(crate) use authorization::OwnerManagementAccess;

#[cfg(test)]
pub(crate) use provisioning_tests::{ready_fixture, with_test_store};

mod sharing_contract;
pub(crate) use sharing_contract::*;

#[cfg(test)]
mod sharing_tests;

mod sharing;
mod sharing_confirmation;

mod rate_limit;

mod native_cleanup;

#[cfg(test)]
mod native_cleanup_tests;
#[cfg(test)]
mod onboarding_tests;
#[cfg(test)]
mod provisioning_fault_tests;
