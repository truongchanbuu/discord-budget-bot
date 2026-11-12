use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    Owner,
    Target,
    Origin,
    Idempotency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CoreError {
    #[error("only VND is supported")]
    UnsupportedCurrency,
    #[error("fractional precision is not supported")]
    UnsupportedPrecision,
    #[error("amount must contain ASCII whole-đồng digits")]
    InvalidAmountFormat,
    #[error("amount must not be negative")]
    NegativeAmount,
    #[error("amount exceeds the supported range")]
    AmountOutOfRange,
    #[error("arithmetic result exceeds the supported range")]
    ArithmeticOutOfRange,
    #[error("arithmetic result would be negative")]
    NegativeResult,
    #[error("change amount must be positive")]
    ZeroChange,
    #[error("reference must not be blank: {0:?}")]
    InvalidReference(ReferenceKind),
    #[error("insufficient virtual funds")]
    InsufficientFunds,
    #[error("prior context does not match the requested scope")]
    PriorContextMismatch,
    #[error("idempotency key conflicts with prior financial meaning")]
    IdempotencyConflict,
    #[error("handoff alone does not establish completion")]
    HandoffNotCompletion,
}
