mod commit;
mod drafts;
mod mapping;
mod queries;
use super::{application::*, domain::*};
use crate::features::{
    financial_core::{Money, OwnerRef},
    ledger::Journal,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct PgStore {
    pool: PgPool,
}
impl PgStore {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub(crate) fn journal(pool: PgPool) -> Journal {
        Journal::new(Arc::new(Self::new(pool)))
    }
}
impl JournalStore for PgStore {
    fn open_draft<'a>(&'a self, input: DraftInput<'a>) -> StoreFuture<'a, Draft> {
        Box::pin(drafts::open(self, input))
    }
    fn draft<'a>(&'a self, owner: &'a OwnerRef, id: i64) -> StoreFuture<'a, Draft> {
        Box::pin(async move { mapping::load_draft(&self.pool, owner, id).await })
    }
    fn act<'a>(&'a self, owner: &'a OwnerRef, mutation: Mutation) -> StoreFuture<'a, ActionResult> {
        Box::pin(commit::apply(self, owner, mutation))
    }
    fn funds<'a>(&'a self, owner: &'a OwnerRef) -> StoreFuture<'a, Vec<(Fund, Money)>> {
        Box::pin(queries::funds(self, owner))
    }
    fn create_fund<'a>(&'a self, owner: &'a OwnerRef, name: String) -> StoreFuture<'a, ()> {
        Box::pin(queries::create_fund(self, owner, name))
    }
    fn summary<'a>(&'a self, owner: &'a OwnerRef, month: Month) -> StoreFuture<'a, Summary> {
        Box::pin(queries::summary(self, owner, month))
    }
}
async fn lock_owner(
    tx: &mut Transaction<'_, Postgres>,
    owner: &OwnerRef,
) -> Result<(), JournalError> {
    sqlx::query("INSERT INTO fg_owners(owner) VALUES($1) ON CONFLICT DO NOTHING")
        .bind(owner.as_str())
        .execute(&mut **tx)
        .await
        .map_err(|_| JournalError::Storage)?;
    sqlx::query("SELECT owner FROM fg_owners WHERE owner=$1 FOR UPDATE")
        .bind(owner.as_str())
        .fetch_one(&mut **tx)
        .await
        .map_err(|_| JournalError::Storage)?;
    sqlx::query(
        r#"
INSERT INTO fg_funds(owner, name, role) VALUES($1, 'chi tiêu', 'spending'), ($1, 'tiết kiệm',
'saving'), ($1, 'đầu tư', 'investment') ON CONFLICT DO NOTHING
"#,
    )
    .bind(owner.as_str())
    .execute(&mut **tx)
    .await
    .map_err(|_| JournalError::Storage)?;
    Ok(())
}
#[cfg(test)]
mod commit_tests;
#[cfg(test)]
mod report_tests;
#[cfg(test)]
mod security_tests;
#[cfg(test)]
mod tests;
