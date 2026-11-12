mod application;
mod domain;

pub use domain::{AccessError, BindingState, DiscordAccountId, TrustedActor};

pub use application::{OwnerBinding, authorize_scope, resolve_owner};
