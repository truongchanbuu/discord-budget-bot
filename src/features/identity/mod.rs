mod application;
mod domain;
pub(crate) mod presentation;

pub use domain::{AccessError, BindingState, DiscordAccountId, TrustedActor};

pub use application::{OwnerBinding, authorize_scope, resolve_owner};
