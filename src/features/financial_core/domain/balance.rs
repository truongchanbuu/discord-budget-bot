use super::{CoreError, Money};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Increase,
    Decrease,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BalanceChange {
    direction: Direction,
    amount: Money,
}

impl BalanceChange {
    pub fn new(direction: Direction, amount: Money) -> Result<Self, CoreError> {
        if amount.whole_dong() == 0 {
            return Err(CoreError::ZeroChange);
        }
        Ok(Self { direction, amount })
    }

    pub fn direction(self) -> Direction {
        self.direction
    }

    pub fn amount(self) -> Money {
        self.amount
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VirtualBalance(Money);

impl VirtualBalance {
    pub fn new(amount: Money) -> Self {
        Self(amount)
    }

    pub fn amount(self) -> Money {
        self.0
    }

    pub fn preview(&self, change: &BalanceChange) -> Result<Self, CoreError> {
        let amount = match change.direction {
            Direction::Increase => self.0.checked_add(change.amount)?,
            Direction::Decrease => self
                .0
                .checked_sub(change.amount)
                .map_err(|_| CoreError::InsufficientFunds)?,
        };
        Ok(Self(amount))
    }
}

impl fmt::Debug for BalanceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BalanceChange([redacted])")
    }
}

impl fmt::Debug for VirtualBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VirtualBalance([redacted])")
    }
}
