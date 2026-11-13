use super::{Entry, Fund, JournalError, Kind, Month, Role, balances};
use chrono::NaiveDate;
use std::collections::BTreeMap;

pub(crate) struct Summary {
    pub opening: u128,
    pub income: u128,
    pub expenses: u128,
    pub closing: u128,
    pub available: u128,
    pub saving: u128,
    pub investment: u128,
    pub categories: BTreeMap<String, u128>,
}
impl std::fmt::Debug for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Summary([redacted])")
    }
}
impl Summary {
    pub(crate) fn percent(&self) -> Result<Option<u128>, JournalError> {
        if self.income == 0 {
            return Ok(None);
        }
        self.expenses
            .checked_mul(100)
            .map(|value| Some(value / self.income))
            .ok_or(JournalError::Overflow)
    }
}

pub(crate) fn summarize(
    funds: &[Fund],
    entries: &[Entry],
    month: Month,
    today: NaiveDate,
) -> Result<Summary, JournalError> {
    let before: Vec<_> = entries
        .iter()
        .filter(|entry| entry.proposal.date < month.first)
        .cloned()
        .collect();
    let through: Vec<_> = entries
        .iter()
        .filter(|entry| entry.proposal.date < month.end)
        .cloned()
        .collect();
    let initial = balances(funds, &before, today)?;
    let final_balances = balances(funds, &through, today)?;
    let total=|values:&BTreeMap<String,crate::features::financial_core::Money>| -> Result<u128,JournalError> {
        values.values().try_fold(0u128,|sum,value|sum.checked_add(u128::from(value.whole_dong())).ok_or(JournalError::Overflow))
    };
    let mut report = Summary {
        opening: total(&initial)?,
        income: 0,
        expenses: 0,
        closing: total(&final_balances)?,
        available: 0,
        saving: 0,
        investment: 0,
        categories: BTreeMap::new(),
    };
    for fund in funds.iter().filter(|fund| fund.role == Role::Spending) {
        report.available = report
            .available
            .checked_add(u128::from(final_balances[&fund.name].whole_dong()))
            .ok_or(JournalError::Overflow)?;
    }
    for entry in entries
        .iter()
        .filter(|entry| entry.proposal.date >= month.first && entry.proposal.date < month.end)
    {
        let value = u128::from(
            entry
                .proposal
                .amount
                .ok_or(JournalError::Storage)?
                .whole_dong(),
        );
        let field = match entry.proposal.kind {
            Kind::Income => Some(&mut report.income),
            Kind::Opening => Some(&mut report.opening),
            Kind::Expense => {
                let category = report
                    .categories
                    .entry(entry.proposal.category.clone())
                    .or_default();
                *category = category.checked_add(value).ok_or(JournalError::Overflow)?;
                Some(&mut report.expenses)
            }
            Kind::Transfer => match funds
                .iter()
                .find(|fund| Some(&fund.name) == entry.proposal.destination.as_ref())
                .map(|fund| fund.role)
            {
                Some(Role::Saving) => Some(&mut report.saving),
                Some(Role::Investment) => Some(&mut report.investment),
                _ => None,
            },
        };
        if let Some(field) = field {
            *field = field.checked_add(value).ok_or(JournalError::Overflow)?;
        }
    }
    Ok(report)
}
