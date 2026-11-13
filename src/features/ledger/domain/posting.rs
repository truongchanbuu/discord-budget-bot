use super::{Entry, Fund, JournalError, Kind};
use crate::features::financial_core::{BalanceChange, CoreError, Direction, Money, VirtualBalance};
use chrono::NaiveDate;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn balances(
    funds: &[Fund],
    entries: &[Entry],
    today: NaiveDate,
) -> Result<BTreeMap<String, Money>, JournalError> {
    let zero = Money::parse("VND", "0").map_err(|_| JournalError::Overflow)?;
    let mut values: BTreeMap<_, _> = funds.iter().map(|fund| (fund.name.clone(), zero)).collect();
    let mut initialized = BTreeSet::new();
    let mut history: Vec<_> = entries.iter().collect();
    history.sort_by_key(|entry| (entry.proposal.date, entry.id));
    for entry in history {
        let proposal = &entry.proposal;
        let amount = proposal.validate(today)?;
        if proposal.kind == Kind::Opening && !initialized.insert(proposal.fund.clone()) {
            return Err(JournalError::OpeningExists);
        }
        let direction = if matches!(proposal.kind, Kind::Expense | Kind::Transfer) {
            Direction::Decrease
        } else {
            Direction::Increase
        };
        move_money(&mut values, &proposal.fund, direction, amount)?;
        if let Some(destination) = &proposal.destination {
            move_money(&mut values, destination, Direction::Increase, amount)?;
        }
    }
    Ok(values)
}

fn move_money(
    values: &mut BTreeMap<String, Money>,
    fund: &str,
    direction: Direction,
    amount: Money,
) -> Result<(), JournalError> {
    let current = values.get_mut(fund).ok_or(JournalError::InvalidInput)?;
    let change = BalanceChange::new(direction, amount).map_err(|_| JournalError::InvalidInput)?;
    *current = VirtualBalance::new(*current)
        .preview(&change)
        .map_err(|error| {
            if error == CoreError::InsufficientFunds {
                JournalError::InsufficientFunds
            } else {
                JournalError::Overflow
            }
        })?
        .amount();
    Ok(())
}
