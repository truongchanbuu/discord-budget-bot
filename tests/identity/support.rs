use fundguard::features::{
    financial_core::OwnerRef,
    identity::{BindingState, DiscordAccountId, OwnerBinding, TrustedActor},
};

pub fn account(value: &str) -> DiscordAccountId {
    DiscordAccountId::parse(value).unwrap()
}

pub fn actor(value: &str) -> TrustedActor {
    TrustedActor::new(account(value))
}

pub fn owner(value: &str) -> OwnerRef {
    OwnerRef::new(value.into()).unwrap()
}

pub fn binding(account_id: &str, owner_id: &str, state: BindingState) -> OwnerBinding {
    OwnerBinding::new(account(account_id), owner(owner_id), state)
}
