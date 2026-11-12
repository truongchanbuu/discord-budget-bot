use super::support::*;
use fundguard::features::financial_core::*;
use std::error::Error;

#[test]
fn every_reference_role_rejects_blank_context() {
    for input in ["", " \t\n", "\u{2003}"] {
        assert_eq!(
            OwnerRef::new(input.into()).err(),
            Some(CoreError::InvalidReference(ReferenceKind::Owner))
        );
        assert_eq!(
            TargetRef::new(input.into()).err(),
            Some(CoreError::InvalidReference(ReferenceKind::Target))
        );
        assert_eq!(
            OriginRef::new(input.into()).err(),
            Some(CoreError::InvalidReference(ReferenceKind::Origin))
        );
        assert_eq!(
            IdempotencyKey::new(input.into()).err(),
            Some(CoreError::InvalidReference(ReferenceKind::Idempotency))
        );
    }
}

#[test]
fn handoff_is_not_completion_and_manual_result_stays_virtual() {
    let mut input = intent();
    input.completion_basis = CompletionBasis::HandoffOnly;
    for current in [0, 1000] {
        assert_eq!(
            validate_intent(&input, balance(current), None).err(),
            Some(CoreError::HandoffNotCompletion)
        );
    }
    input.completion_basis = CompletionBasis::ManualConfirmation;
    let candidate = eligible(&input, 1000);
    assert_eq!(candidate.before(), balance(1000));
    assert_eq!(candidate.proposed(), balance(750));
    assert_eq!(candidate.intent(), &input);
    assert_eq!(candidate.intent().origin.as_str(), "origin-synthetic");
    assert_eq!(
        candidate.intent().completion_basis,
        CompletionBasis::ManualConfirmation
    );
    let Decision::Replay(replay) = validate_intent(&input, balance(0), Some(&candidate)).unwrap()
    else {
        panic!("matching prior must remain replay");
    };
    assert_eq!(replay.intent(), &input);
    assert_eq!(replay.proposed(), candidate.proposed());
}

fn assert_redacted(text: &str) {
    for marker in [
        "owner-synthetic",
        "target-synthetic",
        "origin-synthetic",
        "key-synthetic",
        "987654321",
        "invalid-financial-marker",
    ] {
        assert!(!text.contains(marker));
    }
}

#[test]
fn sensitive_values_and_nested_decisions_have_redacted_debug() {
    let input = intent();
    let candidate = eligible(&input, 1000);
    let decision = validate_intent(&input, balance(0), Some(&candidate)).unwrap();
    let amount = money(987654321);
    let current = balance(987654321);
    let change = BalanceChange::new(Direction::Increase, amount).unwrap();
    for formatted in [
        format!("{amount:?}"),
        format!("{current:#?}"),
        format!("{change:?}"),
        format!("{:?}", input.owner),
        format!("{:#?}", input.target),
        format!("{:?}", input.origin),
        format!("{:#?}", input.key),
        format!("{input:?}"),
        format!("{candidate:#?}"),
        format!("{decision:?}"),
        format!("{:#?}", (Some(&input), Some(&candidate), Some(&decision))),
        format!(
            "{:?}",
            validate_intent(&input, balance(1000), None).unwrap()
        ),
    ] {
        assert_redacted(&formatted);
    }
}

#[test]
fn error_display_debug_and_source_chains_do_not_retain_payloads() {
    let errors = [
        Money::parse("VND", "invalid-financial-marker").unwrap_err(),
        CoreError::UnsupportedCurrency,
        CoreError::UnsupportedPrecision,
        CoreError::NegativeAmount,
        CoreError::AmountOutOfRange,
        CoreError::ArithmeticOutOfRange,
        CoreError::NegativeResult,
        CoreError::ZeroChange,
        CoreError::InvalidReference(ReferenceKind::Owner),
        CoreError::InsufficientFunds,
        CoreError::PriorContextMismatch,
        CoreError::IdempotencyConflict,
        CoreError::HandoffNotCompletion,
    ];
    for error in errors {
        assert_redacted(&format!("{error} {error:?} {error:#?}"));
        let mut source = error.source();
        while let Some(error) = source {
            assert_redacted(&format!("{error} {error:?}"));
            source = error.source();
        }
    }
}
