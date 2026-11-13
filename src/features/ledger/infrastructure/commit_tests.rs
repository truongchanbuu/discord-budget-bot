use super::{tests::fixture, *};
use chrono::{Days, Utc};
use sqlx::Row;

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn concurrent_confirmation_replays_once_and_restart_keeps_history() {
    let f = fixture().await;
    let opening = f.preview("opening", "số dư đầu kỳ 200k").await;
    f.confirm(&opening).await.unwrap();
    let draft = f
        .preview("expense", "donate 100k cho vtuber synthetic")
        .await;
    let (left, right) = tokio::join!(f.confirm(&draft), f.confirm(&draft));
    let (ActionResult::Committed(left), ActionResult::Committed(right)) =
        (left.unwrap(), right.unwrap())
    else {
        panic!("receipts required")
    };
    assert_eq!(left.id, right.id);
    assert_ne!(left.replayed, right.replayed);
    assert_eq!(
        sqlx::query("SELECT count(*) AS count FROM fg_entries WHERE owner=$1 AND origin='expense'")
            .bind(&f.owner)
            .fetch_one(&f.pool)
            .await
            .unwrap()
            .get::<i64, _>("count"),
        1
    );
    sqlx::query("UPDATE fg_drafts SET expires=clock_timestamp()-INTERVAL '1 hour' WHERE id=$1")
        .bind(draft.id)
        .execute(&f.pool)
        .await
        .unwrap();
    assert!(matches!(
        f.confirm(&draft).await.unwrap(),
        ActionResult::Committed(Receipt { replayed: true, .. })
    ));
    let pending = f.preview("pending-restart", "dnt tiền cho vtuber").await;
    f.pool.close().await;
    let pool =
        crate::platform::database::connect(&std::env::var("FUNDGUARD_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
    let restarted = PgStore::journal(pool.clone());
    let restored = restarted
        .draft(f.scope(), pending.id, pending.revision)
        .await
        .unwrap();
    assert_eq!(restored.original_note, pending.original_note);
    assert!(restored.proposal.amount.is_none());
    let funds = restarted.funds(f.scope()).await.unwrap();
    assert_eq!(
        funds
            .iter()
            .find(|(fund, _)| fund.name == "chi tiêu")
            .unwrap()
            .1
            .whole_dong(),
        100_000
    );
    pool.close().await;
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn insufficient_and_backdated_spending_roll_back_whole_effect() {
    let f = fixture().await;
    let today = local_date(Utc::now());
    let earlier = today.checked_sub_days(Days::new(1)).unwrap();
    let opening = f
        .preview("opening", &format!("số dư đầu kỳ 200k ngày {earlier}"))
        .await;
    f.confirm(&opening).await.unwrap();
    let transfer = f.preview("reserve", "tiết kiệm 100k").await;
    f.confirm(&transfer).await.unwrap();
    let bad = f
        .preview("backdated", &format!("chi 150k ngày {earlier}"))
        .await;
    assert_eq!(
        f.confirm(&bad).await.err(),
        Some(JournalError::InsufficientFunds)
    );
    let funds = f.journal.funds(f.scope()).await.unwrap();
    assert_eq!(
        funds
            .iter()
            .find(|(fund, _)| fund.name == "chi tiêu")
            .unwrap()
            .1
            .whole_dong(),
        100_000
    );
    assert_eq!(
        funds
            .iter()
            .find(|(fund, _)| fund.name == "tiết kiệm")
            .unwrap()
            .1
            .whole_dong(),
        100_000
    );
    let again = f.preview("again", "số dư đầu kỳ 1k").await;
    assert_eq!(
        f.confirm(&again).await.err(),
        Some(JournalError::OpeningExists)
    );
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn full_u64_and_database_integral_checks_are_preserved() {
    let f = fixture().await;
    let draft = f.preview("max", &format!("nhận lương {}", u64::MAX)).await;
    for invalid in ["0", "-1", "1.5", "18446744073709551616"] {
        assert!(
            sqlx::query("UPDATE fg_drafts SET amount=$1::text::numeric WHERE id=$2")
                .bind(invalid)
                .bind(draft.id)
                .execute(&f.pool)
                .await
                .is_err()
        );
    }
    sqlx::query("UPDATE fg_drafts SET amount=amount*1.000 WHERE id=$1")
        .bind(draft.id)
        .execute(&f.pool)
        .await
        .unwrap();
    f.confirm(&draft).await.unwrap();
    let extra = f.preview("overflow", "nhận 1").await;
    assert_eq!(f.confirm(&extra).await.err(), Some(JournalError::Overflow));
    assert!(
        sqlx::query("UPDATE fg_entries SET note='changed' WHERE owner=$1")
            .bind(&f.owner)
            .execute(&f.pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM fg_entries WHERE owner=$1")
            .bind(&f.owner)
            .execute(&f.pool)
            .await
            .is_err()
    );
    assert_eq!(
        f.journal
            .funds(f.scope())
            .await
            .unwrap()
            .into_iter()
            .find(|(fund, _)| fund.name == "chi tiêu")
            .unwrap()
            .1
            .whole_dong(),
        u64::MAX
    );
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn failure_after_entry_insert_rolls_back_entry_and_draft_together() {
    let f = fixture().await;
    let opening = f.preview("initial", "số dư đầu kỳ 200k").await;
    f.confirm(&opening).await.unwrap();
    let draft = f.preview("rollback", "tiết kiệm 100k").await;
    sqlx::raw_sql("CREATE TABLE fg_test_failure(owner TEXT PRIMARY KEY); CREATE FUNCTION fg_test_reject() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.status='posted' AND EXISTS(SELECT 1 FROM fg_test_failure WHERE owner=NEW.owner) THEN RAISE EXCEPTION 'controlled storage failure'; END IF; RETURN NEW; END; $$; CREATE TRIGGER fg_test_failure BEFORE UPDATE ON fg_drafts FOR EACH ROW EXECUTE FUNCTION fg_test_reject();").execute(&f.pool).await.unwrap();
    sqlx::query("INSERT INTO fg_test_failure(owner) VALUES($1)")
        .bind(&f.owner)
        .execute(&f.pool)
        .await
        .unwrap();
    assert_eq!(f.confirm(&draft).await.err(), Some(JournalError::Storage));
    let count = sqlx::query(
        "SELECT count(*) AS count FROM fg_entries WHERE owner=$1 AND origin='rollback'",
    )
    .bind(&f.owner)
    .fetch_one(&f.pool)
    .await
    .unwrap()
    .get::<i64, _>("count");
    assert_eq!(count, 0);
    assert_eq!(
        f.journal
            .draft(f.scope(), draft.id, draft.revision)
            .await
            .unwrap()
            .state,
        State::Pending
    );
    sqlx::raw_sql("DROP TRIGGER fg_test_failure ON fg_drafts; DROP FUNCTION fg_test_reject(); DROP TABLE fg_test_failure;").execute(&f.pool).await.unwrap();
    f.confirm(&draft).await.unwrap();
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn distinct_concurrent_expenses_cannot_overspend() {
    let f = fixture().await;
    let opening = f.preview("start", "số dư đầu kỳ 200k").await;
    f.confirm(&opening).await.unwrap();
    let first = f.preview("expense-a", "chi 150k").await;
    let second = f.preview("expense-b", "chi 150k").await;
    let (left, right) = tokio::join!(f.confirm(&first), f.confirm(&second));
    assert_eq!(usize::from(left.is_ok()) + usize::from(right.is_ok()), 1);
    let error = if left.is_err() {
        left.err()
    } else {
        right.err()
    };
    assert_eq!(error, Some(JournalError::InsufficientFunds));
    let funds = f.journal.funds(f.scope()).await.unwrap();
    assert_eq!(
        funds
            .iter()
            .find(|(fund, _)| fund.name == "chi tiêu")
            .unwrap()
            .1
            .whole_dong(),
        50_000
    );
}
