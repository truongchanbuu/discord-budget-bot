use super::*;
use crate::features::finance_spaces::{
    MemberRef,
    application::{Route, classify, leading_mention},
};
#[test]
fn guild_start_and_access_cannot_fall_through_to_financial_parser() {
    let bot = MemberRef::parse("99").unwrap();
    for (text, route) in [
        ("<@99> bắt đầu", Route::Start),
        ("<@99> quyền", Route::Access),
        ("<@99> chi 1k", Route::Finance),
    ] {
        assert_eq!(classify(leading_mention(text, bot, true).unwrap()), route);
        assert!(leading_mention(text, bot, false).is_none());
    }
    assert!(leading_mention("reply-only", bot, true).is_none());
}
#[tokio::test]
async fn failed_private_ack_never_polls_guild_financial_action() {
    let called = AtomicBool::new(false);
    let result = acknowledged_action(async { Err(JournalError::Delivery) }, async {
        called.store(true, Ordering::Relaxed);
        Ok(())
    })
    .await;
    assert_eq!(result, Err(JournalError::Delivery));
    assert!(!called.load(Ordering::Relaxed));
}

#[tokio::test(start_paused = true)]
async fn acknowledged_workflow_returns_safe_timeout_at_twenty_seconds() {
    let start = tokio::time::Instant::now();
    let result = acknowledged_action(
        async { Ok(()) },
        std::future::pending::<Result<(), JournalError>>(),
    )
    .await
    .unwrap();
    assert_eq!(result, Err(JournalError::Timeout));
    assert_eq!(start.elapsed(), Duration::from_secs(20));
}
