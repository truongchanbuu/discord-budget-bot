use super::CoreError;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(u64);

impl Money {
    pub fn parse(currency: &str, quantity: &str) -> Result<Self, CoreError> {
        if currency != "VND" {
            return Err(CoreError::UnsupportedCurrency);
        }
        if quantity.strip_prefix('-').is_some_and(is_unsigned_quantity) {
            return Err(CoreError::NegativeAmount);
        }
        if let Some((whole, fraction)) = quantity.split_once('.')
            && is_unsigned_quantity(whole)
            && is_unsigned_quantity(fraction)
        {
            return Err(CoreError::UnsupportedPrecision);
        }
        if !is_unsigned_quantity(quantity) {
            return Err(CoreError::InvalidAmountFormat);
        }
        quantity
            .parse::<u64>()
            .map(Self)
            .map_err(|_| CoreError::AmountOutOfRange)
    }

    pub fn whole_dong(self) -> u64 {
        self.0
    }

    pub fn checked_add(self, other: Self) -> Result<Self, CoreError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(CoreError::ArithmeticOutOfRange)
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, CoreError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(CoreError::NegativeResult)
    }
}

fn is_unsigned_quantity(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Money([redacted])")
    }
}
