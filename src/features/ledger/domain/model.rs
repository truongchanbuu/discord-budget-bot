use super::JournalError;
use crate::features::financial_core::{Money, OwnerRef};
use chrono::{DateTime, Datelike, Months, NaiveDate, Utc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Income,
    Expense,
    Transfer,
    Opening,
}
impl Kind {
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Income => "income",
            Self::Expense => "expense",
            Self::Transfer => "transfer",
            Self::Opening => "opening",
        }
    }
    pub(crate) fn read(value: &str) -> Result<Self, JournalError> {
        match value {
            "income" => Ok(Self::Income),
            "expense" => Ok(Self::Expense),
            "transfer" => Ok(Self::Transfer),
            "opening" => Ok(Self::Opening),
            _ => Err(JournalError::Storage),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Role {
    Spending,
    Saving,
    Investment,
}
impl Role {
    pub(crate) fn read(value: &str) -> Result<Self, JournalError> {
        match value {
            "spending" => Ok(Self::Spending),
            "saving" => Ok(Self::Saving),
            "investment" => Ok(Self::Investment),
            _ => Err(JournalError::Storage),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Proposal {
    pub kind: Kind,
    pub amount: Option<Money>,
    pub category: String,
    pub fund: String,
    pub destination: Option<String>,
    pub date: NaiveDate,
    pub note: String,
}
impl Proposal {
    pub(crate) fn validate(&self, today: NaiveDate) -> Result<Money, JournalError> {
        checked_note(&self.note)?;
        checked_note(&self.category)?;
        checked_note(&self.fund)?;
        if let Some(destination) = &self.destination {
            checked_note(destination)?;
        }
        let amount = self.amount.ok_or(JournalError::AmountRequired)?;
        if amount.whole_dong() == 0
            || self.date > today
            || name(&self.fund)? != self.fund
            || self.category.trim().is_empty()
            || self.category.encode_utf16().count() > 80
            || self.category.chars().any(char::is_control)
            || self.note.encode_utf16().count() > 1000
            || self.note.chars().any(|c| c == '\0')
        {
            return Err(JournalError::InvalidInput);
        }
        if self.kind == Kind::Expense
            && ["tiết kiệm", "đầu tư", "tiet kiem", "dau tu"]
                .contains(&self.category.to_lowercase().as_str())
        {
            return Err(JournalError::InvalidInput);
        }
        match (&self.kind, &self.destination) {
            (Kind::Transfer, Some(destination))
                if name(destination)? == *destination && *destination != self.fund => {}
            (Kind::Transfer, _) => return Err(JournalError::InvalidInput),
            (_, Some(_)) => return Err(JournalError::InvalidInput),
            _ => {}
        }
        Ok(amount)
    }
}

#[derive(Clone)]
pub(crate) struct Fund {
    pub name: String,
    pub role: Role,
}
#[derive(Clone)]
pub(crate) struct Entry {
    pub id: i64,
    pub proposal: Proposal,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum State {
    Pending,
    Posted,
    Cancelled,
}
#[derive(Clone)]
pub(crate) struct Draft {
    pub id: i64,
    pub owner: OwnerRef,
    pub channel: String,
    pub original_note: String,
    pub proposal: Proposal,
    pub revision: i32,
    pub expires: DateTime<Utc>,
    pub state: State,
    pub entry: Option<i64>,
}
#[derive(Clone)]
pub(crate) struct Receipt {
    pub id: i64,
    pub replayed: bool,
}
#[derive(Clone, Copy)]
pub(crate) struct Month {
    pub first: NaiveDate,
    pub end: NaiveDate,
}
impl Month {
    pub(crate) fn parse(value: &str) -> Result<Self, JournalError> {
        if value.len() != 7 {
            return Err(JournalError::InvalidInput);
        }
        let first = date(&format!("{value}-01"))?;
        let end = first
            .checked_add_months(Months::new(1))
            .ok_or(JournalError::InvalidInput)?;
        Ok(Self { first, end })
    }
    pub(crate) fn current(today: NaiveDate) -> Result<Self, JournalError> {
        Self::parse(&format!("{:04}-{:02}", today.year(), today.month()))
    }
}
pub(crate) fn date(value: &str) -> Result<NaiveDate, JournalError> {
    if value.len() != 10 || !value.is_ascii() {
        return Err(JournalError::InvalidInput);
    }
    let parsed =
        NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| JournalError::InvalidInput)?;
    if parsed.format("%Y-%m-%d").to_string() != value {
        return Err(JournalError::InvalidInput);
    }
    Ok(parsed)
}
pub(crate) fn name(value: &str) -> Result<String, JournalError> {
    checked_note(value)?;
    let value = value.trim().to_lowercase();
    if value.is_empty()
        || value.encode_utf16().count() > 80
        || value.chars().any(char::is_control)
        || value.contains("->")
    {
        return Err(JournalError::InvalidInput);
    }
    Ok(value)
}
pub(crate) fn local_date(now: DateTime<Utc>) -> NaiveDate {
    (now + chrono::Duration::hours(7)).date_naive()
}

macro_rules! redacted {
    ($($value:ty),+) => {$(impl std::fmt::Debug for $value {
        fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.write_str(concat!(stringify!($value),"([redacted])"))}
    })+};
}
redacted!(Proposal, Fund, Entry, Draft, Receipt, Month);

pub(crate) fn checked_note(value: &str) -> Result<(), JournalError> {
    if value.encode_utf16().count() > 1000 || value.contains('\0') {
        return Err(JournalError::InvalidInput);
    }
    let lower = value.to_lowercase();
    let forbidden = lower
        .split(|character: char| !character.is_alphanumeric())
        .any(|word| ["otp", "cvv", "cvc", "password"].contains(&word))
        || [
            "mật khẩu",
            "mat khau",
            "mã pin",
            "ma pin",
            "bank pin",
            "card pin",
            "session_token",
            "banking cookie",
        ]
        .iter()
        .any(|phrase| lower.contains(phrase));
    if forbidden {
        return Err(JournalError::SensitiveInput);
    }
    Ok(())
}
