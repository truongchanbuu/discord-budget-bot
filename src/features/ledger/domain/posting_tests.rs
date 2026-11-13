use super::*;
fn proposal(kind: Kind, value: &str, fund: &str, destination: Option<&str>, day: &str) -> Proposal {
    Proposal {
        kind,
        amount: Some(amount(value).unwrap()),
        category: "Khác".into(),
        fund: fund.into(),
        destination: destination.map(str::to_owned),
        date: date(day).unwrap(),
        note: "synthetic".into(),
    }
}
fn funds() -> Vec<Fund> {
    vec![
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
    ]
}
#[test]
fn transfers_conserve_and_history_rejects_backdated_overdraft() {
    let today = date("2026-11-13").unwrap();
    let mut entries = vec![
        Entry {
            id: 1,
            proposal: proposal(Kind::Income, "200", "chi tiêu", None, "2026-11-10"),
        },
        Entry {
            id: 2,
            proposal: proposal(
                Kind::Transfer,
                "100",
                "chi tiêu",
                Some("tiết kiệm"),
                "2026-11-11",
            ),
        },
    ];
    let values = balances(&funds(), &entries, today).unwrap();
    assert_eq!(values["chi tiêu"].whole_dong(), 100);
    assert_eq!(values["tiết kiệm"].whole_dong(), 100);
    entries.push(Entry {
        id: 3,
        proposal: proposal(Kind::Expense, "150", "chi tiêu", None, "2026-11-10"),
    });
    assert_eq!(
        balances(&funds(), &entries, today).unwrap_err(),
        JournalError::InsufficientFunds
    );
}
#[test]
fn openings_and_full_range_remain_exact() {
    let today = date("2026-11-13").unwrap();
    let mut entries = vec![Entry {
        id: 1,
        proposal: proposal(
            Kind::Opening,
            &u64::MAX.to_string(),
            "chi tiêu",
            None,
            "2026-11-01",
        ),
    }];
    assert_eq!(
        balances(&funds(), &entries, today).unwrap()["chi tiêu"].whole_dong(),
        u64::MAX
    );
    entries.push(Entry {
        id: 2,
        proposal: proposal(Kind::Income, "1", "chi tiêu", None, "2026-11-02"),
    });
    assert_eq!(
        balances(&funds(), &entries, today).unwrap_err(),
        JournalError::Overflow
    );
    entries[1].proposal.kind = Kind::Opening;
    assert_eq!(
        balances(&funds(), &entries, today).unwrap_err(),
        JournalError::OpeningExists
    );
}
