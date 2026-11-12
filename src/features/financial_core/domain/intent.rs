use super::{BalanceChange, IdempotencyKey, OriginRef, OwnerRef, TargetRef};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionBasis {
    HandoffOnly,
    ManualConfirmation,
}

#[derive(Clone, PartialEq, Eq)]
pub struct FinancialIntent {
    pub owner: OwnerRef,
    pub target: TargetRef,
    pub origin: OriginRef,
    pub key: IdempotencyKey,
    pub change: BalanceChange,
    pub completion_basis: CompletionBasis,
}

impl fmt::Debug for FinancialIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FinancialIntent([redacted])")
    }
}
