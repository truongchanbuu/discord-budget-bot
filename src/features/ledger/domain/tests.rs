use super::*;

#[test]
fn complete_and_incomplete_vietnamese_inputs_are_reviewable() {
    let today = date("2026-11-13").unwrap();
    let donate = parse("tôi vừa donate 100k cho vtuber A", today).unwrap();
    assert_eq!(donate.kind, Kind::Expense);
    assert_eq!(donate.amount.unwrap().whole_dong(), 100_000);
    assert_eq!(donate.category, "Giải trí");
    assert!(
        parse("tôi vừa dnt tiền cho vtuber", today)
            .unwrap()
            .amount
            .is_none()
    );
    assert_eq!(parse("nhận lương 15tr", today).unwrap().kind, Kind::Income);
    assert_eq!(
        parse("tiết kiệm 3tr", today)
            .unwrap()
            .destination
            .as_deref(),
        Some("tiết kiệm")
    );
    assert_eq!(
        parse("số dư đầu kỳ 2tr", today).unwrap().kind,
        Kind::Opening
    );
    assert!(parse("sẽ nhận lương 15tr", today).is_err());
    assert!(parse("xin chào", today).is_err());
}

#[test]
fn amounts_never_use_float_or_guess_ambiguous_values() {
    assert_eq!(
        amount("18446744073709551615").unwrap().whole_dong(),
        u64::MAX
    );
    for value in ["0", "-1", "1.5tr", "1,5tr", "18446744073709551616", "1e3"] {
        assert!(amount(value).is_err(), "{value}");
    }
    let today = date("2026-11-13").unwrap();
    assert!(parse("chi 100k và 200k", today).unwrap().amount.is_none());
    assert_eq!(
        parse("chi 100 nghìn", today)
            .unwrap()
            .amount
            .unwrap()
            .whole_dong(),
        100_000
    );
    assert!(parse("chi 100k ngày 2026-11-14", today).is_err());
    assert_eq!(
        parse("chi 100k ngày 2026-10-31", today).unwrap().date,
        date("2026-10-31").unwrap()
    );
}

#[test]
fn validation_and_formats_protect_values() {
    let today = date("2026-11-13").unwrap();
    let mut proposal = parse("chi 100k sensitive-note-marker", today).unwrap();
    assert!(!format!("{proposal:?} {proposal:#?}").contains("marker"));
    proposal.category = "tiết kiệm".into();
    assert!(proposal.validate(today).is_err());
    for value in [
        "2026-02-30",
        "2026- 1-03",
        "2026-01- 3",
        "2026-01-3 ",
        "2026-1-03",
    ] {
        assert!(date(value).is_err(), "{value}");
    }
    assert!(Month::parse("2026-13").is_err());
    assert_eq!(name("  Du LỊCH  ").unwrap(), "du lịch");
}

#[test]
fn obvious_banking_secrets_are_rejected_before_draft_storage() {
    let today = date("2026-11-13").unwrap();
    for text in [
        "chi 100k OTP 123456",
        "chi 100k mật khẩu secret",
        "chi 100k CVV 123",
        "chi 100k mã PIN 1234",
    ] {
        assert_eq!(
            parse(text, today).unwrap_err(),
            JournalError::SensitiveInput
        );
    }
    assert!(parse("chi 100k ăn hotpot", today).is_ok());
    for value in ["100nghìn", "100 nghìn", "100k"] {
        assert_eq!(amount(value).unwrap().whole_dong(), 100_000);
    }
}

#[test]
fn ambiguous_multi_action_or_explicit_route_is_not_guessed() {
    let today = date("2026-11-13").unwrap();
    let transfer = parse("chuyển 100k từ tiết kiệm sang chi tiêu", today).unwrap();
    assert!(transfer.destination.is_none());
    assert!(parse("nhận lương 15tr để tiết kiệm", today).is_err());
    assert!(parse("chi 100 USD", today).is_err());
}
