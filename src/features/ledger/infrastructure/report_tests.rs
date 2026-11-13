use super::{tests::fixture, *};
use chrono::{Days, Utc};
#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn monthly_report_and_fund_register_reconcile_durably() {
    let f = fixture().await;
    for (source, input) in [
        ("income", "nhận lương 16tr"),
        ("expense", "chi 6tr ăn uống"),
        ("saving", "tiết kiệm 3tr"),
        ("investment", "đầu tư 2tr"),
    ] {
        let draft = f.preview(source, input).await;
        f.confirm(&draft).await.unwrap();
    }
    let month = Month::current(local_date(Utc::now())).unwrap();
    let report = f.journal.summary(f.scope(), month).await.unwrap();
    assert_eq!(
        (
            report.opening,
            report.income,
            report.expenses,
            report.closing,
            report.available
        ),
        (0, 16_000_000, 6_000_000, 10_000_000, 5_000_000)
    );
    assert_eq!((report.saving, report.investment), (3_000_000, 2_000_000));
    assert_eq!(report.categories.values().sum::<u128>(), report.expenses);
    f.journal.create_fund(f.scope(), "Du LỊCH").await.unwrap();
    assert_eq!(
        f.journal.create_fund(f.scope(), "du lịch").await.err(),
        Some(JournalError::Duplicate)
    );
    assert_eq!(f.journal.funds(f.scope()).await.unwrap().len(), 4);
    let next = Month::current(month.end).unwrap();
    let carry = f.journal.summary(f.scope(), next).await.unwrap();
    assert_eq!(
        (carry.opening, carry.income, carry.expenses, carry.closing),
        (10_000_000, 0, 0, 10_000_000)
    );
}

#[tokio::test]
#[ignore = "requires disposable PostgreSQL; run scripts/test-postgres.sh"]
async fn in_period_initialization_is_not_earned_income() {
    let f = fixture().await;
    let draft = f.preview("opening", "số dư đầu kỳ 2tr").await;
    f.confirm(&draft).await.unwrap();
    let month = Month::current(local_date(Utc::now())).unwrap();
    let report = f.journal.summary(f.scope(), month).await.unwrap();
    assert_eq!(
        (
            report.opening,
            report.income,
            report.expenses,
            report.closing
        ),
        (2_000_000, 0, 0, 2_000_000)
    );
    assert_eq!(report.percent().unwrap(), None);
    let prior = Month::current(month.first.checked_sub_days(Days::new(1)).unwrap()).unwrap();
    let report = f.journal.summary(f.scope(), prior).await.unwrap();
    assert_eq!(report.closing, 0);
}
