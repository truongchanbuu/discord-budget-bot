use super::domain::{AccessError, BindingState, DiscordAccountId, TrustedActor};
use crate::features::financial_core::OwnerRef;
use std::fmt;

pub struct OwnerBinding {
    account: DiscordAccountId,
    owner: OwnerRef,
    state: BindingState,
}

impl OwnerBinding {
    pub(crate) fn configured_account(&self) -> DiscordAccountId {
        self.account
    }

    pub fn new(account: DiscordAccountId, owner: OwnerRef, state: BindingState) -> Self {
        Self {
            account,
            owner,
            state,
        }
    }
}

pub fn resolve_owner<'binding>(
    actor: Option<&TrustedActor>,
    binding: Option<&'binding OwnerBinding>,
) -> Result<&'binding OwnerRef, AccessError> {
    let actor = actor.ok_or(AccessError::Denied)?;
    let binding = binding.ok_or(AccessError::Denied)?;
    if binding.state != BindingState::Enabled || actor.account() != binding.account {
        return Err(AccessError::Denied);
    }
    Ok(&binding.owner)
}

impl fmt::Debug for OwnerBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OwnerBinding([redacted])")
    }
}

pub fn authorize_scope<'binding>(
    actor: Option<&TrustedActor>,
    binding: Option<&'binding OwnerBinding>,
    resource_owner: Option<&OwnerRef>,
) -> Result<&'binding OwnerRef, AccessError> {
    let owner = resolve_owner(actor, binding)?;
    if resource_owner != Some(owner) {
        return Err(AccessError::Denied);
    }
    Ok(owner)
}
