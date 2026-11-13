use super::*;
#[test]
fn monthly_example_reconciles_and_preserves_carryover() {
    let today = date("2026-11-13").unwrap();
    let funds = vec![
        Fund {
            name: "chi tiêu".into(),
            role: Role::Spending,
        },
        Fund {
            name: "tiết kiệm".into(),
            role: Role::Saving,
        },
        Fund {
            name: "đầu tư".into(),
            role: Role::Investment,
        },
    ];
    let cases = [
        (Kind::Income, "16tr", None, "2026-11-01"),
        (Kind::Expense, "6tr", None, "2026-11-02"),
        (Kind::Transfer, "3tr", Some("tiết kiệm"), "2026-11-03"),
        (Kind::Transfer, "2tr", Some("đầu tư"), "2026-11-04"),
    ];
    let entries: Vec<_> = cases
        .into_iter()
        .enumerate()
        .map(|(i, (kind, value, destination, day))| Entry {
            id: i as i64 + 1,
            proposal: Proposal {
                kind,
                amount: Some(amount(value).unwrap()),
                category: "Khác".into(),
                fund: "chi tiêu".into(),
                destination: destination.map(str::to_owned),
                date: date(day).unwrap(),
                note: "synthetic".into(),
            },
        })
        .collect();
    let report = summarize(&funds, &entries, Month::parse("2026-11").unwrap(), today).unwrap();
    assert_eq!(
        (
            report.income,
            report.expenses,
            report.closing,
            report.available
        ),
        (16_000_000, 6_000_000, 10_000_000, 5_000_000)
    );
    assert_eq!(report.percent().unwrap(), Some(37));
    let next = summarize(&funds, &entries, Month::parse("2026-12").unwrap(), today).unwrap();
    assert_eq!(
        (next.opening, next.income, next.expenses, next.closing),
        (10_000_000, 0, 0, 10_000_000)
    );
    assert_eq!(next.percent().unwrap(), None);
}
