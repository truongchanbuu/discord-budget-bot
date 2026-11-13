use crate::features::{
    financial_core::{Money, OwnerRef},
    ledger::domain::{Draft, Fund, JournalError, Month, Proposal, Receipt, Summary},
};
use chrono::{DateTime, Utc};
use std::{future::Future, pin::Pin};
pub(crate) type StoreFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, JournalError>> + Send + 'a>>;

pub(crate) struct DraftInput<'a> {
    pub owner: &'a OwnerRef,
    pub actor: String,
    pub channel: &'a str,
    pub origin: &'a str,
    pub proposal: Proposal,
    pub now: DateTime<Utc>,
}
pub(crate) enum Action {
    Confirm,
    Cancel,
    Edit(Proposal),
}
pub(crate) struct Mutation {
    pub id: i64,
    pub revision: i32,
    pub channel: String,
    pub action: Action,
    pub actor: String,
}
pub(crate) enum ActionResult {
    Preview(Box<Draft>),
    Committed(Receipt),
    Cancelled,
}

pub(crate) trait JournalStore: Send + Sync {
    fn open_draft<'a>(&'a self, input: DraftInput<'a>) -> StoreFuture<'a, Draft>;
    fn draft<'a>(&'a self, owner: &'a OwnerRef, id: i64) -> StoreFuture<'a, Draft>;
    fn act<'a>(&'a self, owner: &'a OwnerRef, mutation: Mutation) -> StoreFuture<'a, ActionResult>;
    fn funds<'a>(&'a self, owner: &'a OwnerRef) -> StoreFuture<'a, Vec<(Fund, Money)>>;
    fn create_fund<'a>(&'a self, owner: &'a OwnerRef, name: String) -> StoreFuture<'a, ()>;
    fn summary<'a>(&'a self, owner: &'a OwnerRef, month: Month) -> StoreFuture<'a, Summary>;
}
