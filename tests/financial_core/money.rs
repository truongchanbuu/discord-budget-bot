use fundguard::features::financial_core::{CoreError, Money};

#[test]
fn exact_vnd_values_and_comparison() {
    let amount = Money::parse("VND", "1000").unwrap();
    let extra = Money::parse("VND", "250").unwrap();
    assert_eq!(amount.checked_add(extra).unwrap().whole_dong(), 1250);
    assert_eq!(amount.checked_sub(extra).unwrap().whole_dong(), 750);
    assert!(amount > extra);
    assert_eq!(Money::parse("VND", "001000").unwrap(), amount);
    for input in ["0", "000"] {
        assert_eq!(Money::parse("VND", input).unwrap().whole_dong(), 0);
    }
}

#[test]
fn quantity_grammar_and_error_order() {
    for (input, error) in [
        ("-1", CoreError::NegativeAmount),
        ("-0", CoreError::NegativeAmount),
        ("0.5", CoreError::UnsupportedPrecision),
        ("1.0", CoreError::UnsupportedPrecision),
        ("", CoreError::InvalidAmountFormat),
        ("+1", CoreError::InvalidAmountFormat),
        (" 1", CoreError::InvalidAmountFormat),
        ("1 ", CoreError::InvalidAmountFormat),
        ("1,000", CoreError::InvalidAmountFormat),
        ("1e3", CoreError::InvalidAmountFormat),
        ("1.2.3", CoreError::InvalidAmountFormat),
        ("-0.5", CoreError::InvalidAmountFormat),
        ("１２", CoreError::InvalidAmountFormat),
        ("١", CoreError::InvalidAmountFormat),
        (".5", CoreError::InvalidAmountFormat),
        ("1.", CoreError::InvalidAmountFormat),
    ] {
        assert_eq!(Money::parse("VND", input).err(), Some(error));
    }
    for currency in ["USD", "vnd", " VND", ""] {
        assert_eq!(
            Money::parse(currency, "bad").err(),
            Some(CoreError::UnsupportedCurrency)
        );
    }
}

#[test]
fn range_and_arithmetic_fail_without_wrapping() {
    let max = Money::parse("VND", "18446744073709551615").unwrap();
    let one = Money::parse("VND", "1").unwrap();
    let zero = Money::parse("VND", "0").unwrap();
    assert_eq!(max.whole_dong(), u64::MAX);
    assert_eq!(
        Money::parse("VND", "18446744073709551616").err(),
        Some(CoreError::AmountOutOfRange)
    );
    assert_eq!(
        max.checked_add(one).err(),
        Some(CoreError::ArithmeticOutOfRange)
    );
    assert_eq!(zero.checked_sub(one).err(), Some(CoreError::NegativeResult));
    assert_eq!(max.checked_add(zero).unwrap(), max);
}
