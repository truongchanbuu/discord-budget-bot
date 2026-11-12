use fundguard::features::financial_core::{
    BalanceChange, CoreError, Direction, Money, VirtualBalance,
};

fn money(value: u64) -> Money {
    Money::parse("VND", &value.to_string()).unwrap()
}

#[test]
fn preview_preserves_balance_and_allows_exact_depletion() {
    let before = VirtualBalance::new(money(1000));
    let change = BalanceChange::new(Direction::Decrease, money(250)).unwrap();
    assert_eq!(before.preview(&change).unwrap().amount(), money(750));
    assert_eq!(before.amount(), money(1000));
    assert_eq!(change.amount(), money(250));
    let depletion = BalanceChange::new(Direction::Decrease, money(1000)).unwrap();
    assert_eq!(before.preview(&depletion).unwrap().amount(), money(0));
    let excess = BalanceChange::new(Direction::Decrease, money(1001)).unwrap();
    assert_eq!(
        before.preview(&excess).err(),
        Some(CoreError::InsufficientFunds)
    );
}

#[test]
fn invalid_changes_and_overflow_fail_explicitly() {
    for direction in [Direction::Increase, Direction::Decrease] {
        assert_eq!(
            BalanceChange::new(direction, money(0)).err(),
            Some(CoreError::ZeroChange)
        );
    }
    let max = VirtualBalance::new(money(u64::MAX));
    let change = BalanceChange::new(Direction::Increase, money(1)).unwrap();
    assert_eq!(
        max.preview(&change).err(),
        Some(CoreError::ArithmeticOutOfRange)
    );
    assert_eq!(max.amount(), money(u64::MAX));
    assert_eq!(
        Money::parse("VND", "-1").err(),
        Some(CoreError::NegativeAmount)
    );
}

#[test]
fn small_exhaustive_previews_match_exact_arithmetic() {
    for initial in 0..=32 {
        for amount in 1..=32 {
            let before = VirtualBalance::new(money(initial));
            let increase = BalanceChange::new(Direction::Increase, money(amount)).unwrap();
            assert_eq!(
                before.preview(&increase).unwrap().amount().whole_dong(),
                initial + amount
            );
            let decrease = BalanceChange::new(Direction::Decrease, money(amount)).unwrap();
            if amount <= initial {
                assert_eq!(
                    before.preview(&decrease).unwrap().amount().whole_dong(),
                    initial - amount
                );
            } else {
                assert_eq!(
                    before.preview(&decrease).err(),
                    Some(CoreError::InsufficientFunds)
                );
            }
            assert_eq!(before.amount().whole_dong(), initial);
        }
    }
}
