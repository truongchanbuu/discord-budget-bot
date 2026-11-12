use super::support::*;
use fundguard::features::financial_core::*;

#[test]
fn new_eligibility_owns_coherent_virtual_snapshot() {
    let mut input = intent();
    let prior = eligible(&input, 1000);
    assert_eq!(prior.before(), balance(1000));
    assert_eq!(prior.proposed(), balance(750));
    assert_eq!(prior.intent(), &input);
    input.target = TargetRef::new("changed-target".into()).unwrap();
    assert_eq!(prior.intent().target.as_str(), "target-synthetic");
}

#[test]
fn matching_replay_borrows_prior_before_capacity_checks() {
    let input = intent();
    let prior = eligible(&input, 1000);
    match validate_intent(&input, balance(0), Some(&prior)).unwrap() {
        Decision::Replay(result) => {
            assert!(std::ptr::eq(result, &prior));
            assert_eq!(result.proposed(), balance(750));
        }
        Decision::Eligible(_) => panic!("retry must not propose another effect"),
    }
    let mut equal_amount = input.clone();
    equal_amount.change =
        BalanceChange::new(Direction::Decrease, Money::parse("VND", "00250").unwrap()).unwrap();
    assert!(matches!(
        validate_intent(&equal_amount, balance(0), Some(&prior)),
        Ok(Decision::Replay(_))
    ));
}

#[test]
fn each_changed_meaning_conflicts_before_fresh_checks() {
    let input = intent();
    let prior = eligible(&input, 1000);
    let changed = [
        FinancialIntent {
            target: TargetRef::new("other-target".into()).unwrap(),
            ..input.clone()
        },
        FinancialIntent {
            origin: OriginRef::new("other-origin".into()).unwrap(),
            ..input.clone()
        },
        FinancialIntent {
            change: BalanceChange::new(Direction::Decrease, money(251)).unwrap(),
            ..input.clone()
        },
        FinancialIntent {
            change: BalanceChange::new(Direction::Increase, money(250)).unwrap(),
            ..input.clone()
        },
        FinancialIntent {
            completion_basis: CompletionBasis::HandoffOnly,
            ..input.clone()
        },
    ];
    for candidate in changed {
        assert_eq!(
            validate_intent(&candidate, balance(0), Some(&prior)).err(),
            Some(CoreError::IdempotencyConflict)
        );
    }
    assert_eq!(prior.intent(), &input);
    assert_eq!(prior.before(), balance(1000));
}

#[test]
fn owner_or_key_mismatch_is_not_absence_or_a_conflict() {
    let input = intent();
    let prior = eligible(&input, 1000);
    let changed = [
        FinancialIntent {
            owner: OwnerRef::new("other-owner".into()).unwrap(),
            ..input.clone()
        },
        FinancialIntent {
            key: IdempotencyKey::new("other-key".into()).unwrap(),
            ..input.clone()
        },
    ];
    for mut candidate in changed {
        candidate.completion_basis = CompletionBasis::HandoffOnly;
        assert_eq!(
            validate_intent(&candidate, balance(0), Some(&prior)).err(),
            Some(CoreError::PriorContextMismatch)
        );
    }
}

#[test]
fn absent_prior_context_proves_no_durable_uniqueness() {
    let input = intent();
    assert!(matches!(
        validate_intent(&input, balance(1000), None),
        Ok(Decision::Eligible(_))
    ));
    assert!(matches!(
        validate_intent(&input, balance(1000), None),
        Ok(Decision::Eligible(_))
    ));
}
