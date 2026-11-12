use fundguard::features::financial_core::*;

pub fn money(value: u64) -> Money {
    Money::parse("VND", &value.to_string()).unwrap()
}

pub fn balance(value: u64) -> VirtualBalance {
    VirtualBalance::new(money(value))
}

pub fn intent() -> FinancialIntent {
    FinancialIntent {
        owner: OwnerRef::new("owner-synthetic".into()).unwrap(),
        target: TargetRef::new("target-synthetic".into()).unwrap(),
        origin: OriginRef::new("origin-synthetic".into()).unwrap(),
        key: IdempotencyKey::new("key-synthetic".into()).unwrap(),
        change: BalanceChange::new(Direction::Decrease, money(250)).unwrap(),
        completion_basis: CompletionBasis::ManualConfirmation,
    }
}

pub fn eligible(intent: &FinancialIntent, starting: u64) -> ValidatedChange {
    match validate_intent(intent, balance(starting), None).unwrap() {
        Decision::Eligible(result) => result,
        Decision::Replay(_) => panic!("no prior context must produce new eligibility"),
    }
}
