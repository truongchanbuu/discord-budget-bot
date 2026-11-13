use super::*;
use crate::features::{
    financial_core::OwnerRef,
    identity::{BindingState, DiscordAccountId, OwnerBinding, TrustedActor},
    ledger::application::{Action, ActionResult, Scope},
};
use chrono::Utc;
use std::{
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
static NEXT: AtomicU64 = AtomicU64::new(0);
pub(super) struct Fixture {
    pub journal: crate::features::ledger::Journal,
    pub pool: PgPool,
    pub actor: TrustedActor,
    pub binding: OwnerBinding,
    pub owner: String,
}
impl Fixture {
    pub fn scope(&self) -> Scope<'_> {
        Scope {
            actor: &self.actor,
            binding: &self.binding,
            channel: "100",
        }
    }
    pub async fn preview(&self, source: &str, input: &str) -> Draft {
        self.journal
            .preview(self.scope(), source, input, Utc::now())
            .await
            .unwrap()
    }
    pub async fn confirm(&self, draft: &Draft) -> Result<ActionResult, JournalError> {
        self.journal
            .act(self.scope(), draft.id, draft.revision, Action::Confirm)
            .await
    }
}
pub(super) async fn fixture() -> Fixture {
    assert_eq!(
        std::env::var("FUNDGUARD_TEST_PRIVATE_CLUSTER").unwrap(),
        "1"
    );
    let url = std::env::var("FUNDGUARD_TEST_DATABASE_URL").unwrap();
    let options = sqlx::postgres::PgConnectOptions::from_str(&url).unwrap();
    assert!(
        options.get_host().starts_with("/tmp/fundguard")
            || ["localhost", "127.0.0.1"].contains(&options.get_host())
    );
    assert!(
        options
            .get_database()
            .unwrap_or("")
            .starts_with("fundguard_test")
    );
    let pool = crate::platform::database::connect(&url).await.unwrap();
    let owner = format!(
        "fixture-{}-{}",
        Utc::now().timestamp_nanos_opt().unwrap(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    );
    let account = DiscordAccountId::parse("123").unwrap();
    let binding = OwnerBinding::new(
        account,
        OwnerRef::new(owner.clone()).unwrap(),
        BindingState::Enabled,
    );
    let journal = crate::features::ledger::Journal::new(Arc::new(PgStore::new(pool.clone())));
    Fixture {
        journal,
        pool,
        actor: TrustedActor::new(account),
        binding,
        owner,
    }
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn durable_draft_revision_cancel_expiry_and_owner_isolation() {
    let f = fixture().await;
    let draft = f.preview("dm:1", "tôi vừa dnt tiền cho vtuber").await;
    assert!(draft.proposal.amount.is_none());
    let same = f.preview("dm:1", "chi 100k changed-marker").await;
    assert_eq!(same.id, draft.id);
    assert_eq!(same.original_note, draft.original_note);
    assert_eq!(
        f.confirm(&draft).await.err(),
        Some(JournalError::AmountRequired)
    );
    let mut proposal = draft.proposal.clone();
    proposal.amount = Some(super::super::domain::amount("100k").unwrap());
    let changed = f
        .journal
        .act(f.scope(), draft.id, 0, Action::Edit(proposal))
        .await
        .unwrap();
    let ActionResult::Preview(changed) = changed else {
        panic!("preview required")
    };
    assert_eq!(changed.revision, 1);
    assert_eq!(f.confirm(&draft).await.err(), Some(JournalError::Stale));
    let foreign = fixture().await;
    assert_eq!(
        foreign
            .journal
            .draft(foreign.scope(), draft.id, 1)
            .await
            .unwrap_err(),
        JournalError::Denied
    );
    let wrong_channel = Scope {
        actor: &f.actor,
        binding: &f.binding,
        channel: "101",
    };
    assert_eq!(
        f.journal
            .draft(wrong_channel, draft.id, 1)
            .await
            .unwrap_err(),
        JournalError::Denied
    );
    f.journal
        .act(f.scope(), draft.id, 1, Action::Cancel)
        .await
        .unwrap();
    assert_eq!(
        f.confirm(&changed).await.err(),
        Some(JournalError::Cancelled)
    );
    let expired = f.preview("dm:2", "chi 1k").await;
    sqlx::query("UPDATE fg_drafts SET expires=clock_timestamp()-INTERVAL '1 second' WHERE id=$1")
        .bind(expired.id)
        .execute(&f.pool)
        .await
        .unwrap();
    assert_eq!(f.confirm(&expired).await.err(), Some(JournalError::Expired));
}
