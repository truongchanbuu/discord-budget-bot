use super::domain::{CompletionBasis, CoreError, FinancialIntent, VirtualBalance};
use std::fmt;

pub struct ValidatedChange {
    intent: FinancialIntent,
    before: VirtualBalance,
    proposed: VirtualBalance,
}

impl ValidatedChange {
    pub fn intent(&self) -> &FinancialIntent {
        &self.intent
    }

    pub fn before(&self) -> VirtualBalance {
        self.before
    }

    pub fn proposed(&self) -> VirtualBalance {
        self.proposed
    }
}

pub enum Decision<'prior> {
    Eligible(ValidatedChange),
    Replay(&'prior ValidatedChange),
}

pub fn validate_intent<'prior>(
    intent: &FinancialIntent,
    current: VirtualBalance,
    prior: Option<&'prior ValidatedChange>,
) -> Result<Decision<'prior>, CoreError> {
    if let Some(prior) = prior {
        if intent.owner != prior.intent.owner || intent.key != prior.intent.key {
            return Err(CoreError::PriorContextMismatch);
        }
        if intent != &prior.intent {
            return Err(CoreError::IdempotencyConflict);
        }
        return Ok(Decision::Replay(prior));
    }
    if intent.completion_basis == CompletionBasis::HandoffOnly {
        return Err(CoreError::HandoffNotCompletion);
    }
    let proposed = current.preview(&intent.change)?;
    Ok(Decision::Eligible(ValidatedChange {
        intent: intent.clone(),
        before: current,
        proposed,
    }))
}

impl fmt::Debug for ValidatedChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidatedChange([redacted])")
    }
}

impl fmt::Debug for Decision<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Eligible(_) => "Decision::Eligible([redacted])",
            Self::Replay(_) => "Decision::Replay([redacted])",
        })
    }
}
