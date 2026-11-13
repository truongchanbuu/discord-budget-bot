use super::{JournalError, Kind, Proposal, checked_note, date};
use crate::features::financial_core::Money;
use chrono::NaiveDate;

pub(crate) fn amount(value: &str) -> Result<Money, JournalError> {
    if value.len() > 32 {
        return Err(JournalError::InvalidInput);
    }
    let value = value.trim().to_lowercase();
    let (digits, multiplier) = [
        ("triệu", 1_000_000u64),
        ("trieu", 1_000_000),
        ("tr", 1_000_000),
        ("nghìn", 1_000),
        ("ngàn", 1_000),
        ("nghin", 1_000),
        ("ngan", 1_000),
        ("k", 1_000),
    ]
    .into_iter()
    .find_map(|(suffix, multiple)| {
        value
            .strip_suffix(suffix)
            .map(|digits| (digits.trim(), multiple))
    })
    .unwrap_or((value.as_str(), 1));
    let base = Money::parse("VND", digits).map_err(|_| JournalError::InvalidInput)?;
    let value = base
        .whole_dong()
        .checked_mul(multiplier)
        .ok_or(JournalError::Overflow)?;
    if value == 0 {
        return Err(JournalError::InvalidInput);
    }
    Money::parse("VND", &value.to_string()).map_err(|_| JournalError::Overflow)
}

pub(crate) fn parse(input: &str, today: NaiveDate) -> Result<Proposal, JournalError> {
    if input.encode_utf16().count() > 1000 || input.contains('\0') {
        return Err(JournalError::InvalidInput);
    }
    checked_note(input)?;
    let lower = input.to_lowercase();
    if ["usd", "eur", "usdt", "btc", "jpy", "$", "€", "¥"]
        .iter()
        .any(|currency| lower.contains(currency))
    {
        return Err(JournalError::InvalidInput);
    }
    if ["sẽ", "dự kiến", "chưa", "du kien", "chua", "se nhan"]
        .iter()
        .any(|word| lower.contains(word))
    {
        return Err(JournalError::InvalidInput);
    }
    let contains = |words: &[&str]| words.iter().any(|word| lower.contains(word));
    if contains(&["nhận", "nhan", "lương", "luong", "freelance"])
        && contains(&[
            "tiết kiệm",
            "tiet kiem",
            "đầu tư",
            "dau tu",
            "chuyển",
            "chuyen",
        ])
    {
        return Err(JournalError::InvalidInput);
    }
    let kind = if contains(&["số dư đầu kỳ", "so du dau ky"]) {
        Kind::Opening
    } else if contains(&[
        "tiết kiệm",
        "tiet kiem",
        "đầu tư",
        "dau tu",
        "chuyển",
        "chuyen",
    ]) {
        Kind::Transfer
    } else if contains(&[
        "nhận",
        "nhan",
        "lương",
        "luong",
        "freelance",
        "thưởng",
        "thuong",
    ]) {
        Kind::Income
    } else if contains(&[
        "chi ", "mua ", "ăn ", "an ", "donate", "dnt", "trả ", "tra ",
    ]) {
        Kind::Expense
    } else {
        return Err(JournalError::InvalidInput);
    };
    let mut category = "Khác";
    for (words, label) in [
        (&["donate", "dnt", "vtuber", "game", "phim"][..], "Giải trí"),
        (&["ăn", "uống", "an ", "uong"][..], "Ăn uống"),
        (&["xe", "taxi", "grab", "xăng", "xang"][..], "Di chuyển"),
        (&["học", "hoc", "sách", "sach"][..], "Học tập"),
        (&["thuốc", "thuoc", "khám", "kham"][..], "Sức khỏe"),
        (&["mua"][..], "Mua sắm"),
        (&["điện", "nước", "internet", "hoa don"][..], "Hóa đơn"),
    ] {
        if contains(words) {
            category = label;
            break;
        }
    }
    if kind == Kind::Income {
        category = if contains(&["lương", "luong"]) {
            "Lương"
        } else if contains(&["freelance"]) {
            "Freelance"
        } else if contains(&["thưởng", "thuong"]) {
            "Thưởng"
        } else {
            "Khác"
        };
    }
    let tokens: Vec<_> = lower.split_whitespace().collect();
    let mut numbers = Vec::new();
    let mut entry_date = today;
    for (index, token) in tokens.iter().enumerate() {
        if index > 0 && ["ngày", "ngay"].contains(&tokens[index - 1]) {
            entry_date = date(token)?;
            continue;
        }
        if token
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_digit() || byte == b'-')
        {
            let mut quantity = token.trim_end_matches(['.', ',', '!', '?']).to_string();
            if let Some(suffix) = tokens.get(index + 1) {
                if ["nghìn", "ngàn", "nghin", "ngan"].contains(suffix) {
                    quantity.push('k');
                } else if ["triệu", "trieu"].contains(suffix) {
                    quantity.push_str("tr");
                }
            }
            numbers.push(amount(&quantity));
        }
    }
    if entry_date > today {
        return Err(JournalError::InvalidInput);
    }
    let amount = if numbers.len() == 1 {
        numbers.pop().and_then(Result::ok)
    } else {
        None
    };
    let destination = if kind == Kind::Transfer && !contains(&[" từ ", " tu ", " sang ", "->"]) {
        if contains(&["tiết kiệm", "tiet kiem"]) {
            Some("tiết kiệm".into())
        } else if contains(&["đầu tư", "dau tu"]) {
            Some("đầu tư".into())
        } else {
            None
        }
    } else {
        None
    };
    Ok(Proposal {
        kind,
        amount,
        category: category.into(),
        fund: "chi tiêu".into(),
        destination,
        date: entry_date,
        note: input.into(),
    })
}
