use super::{reference, serenity};
use crate::features::ledger::domain::{Draft, JournalError};
use std::collections::BTreeMap;

pub(super) fn builder(draft: &Draft) -> serenity::CreateModal {
    let proposal = &draft.proposal;
    let route = proposal
        .destination
        .as_ref()
        .map(|target| format!("{} -> {target}", proposal.fund))
        .unwrap_or_else(|| proposal.fund.clone());
    let amount = proposal
        .amount
        .map(|value| value.whole_dong().to_string())
        .unwrap_or_default();
    let fields = [
        ("amount", "Số tiền VND (số nguyên, k hoặc tr)", amount, 32),
        ("category", "Danh mục", proposal.category.clone(), 80),
        ("route", "Quỹ hoặc nguồn -> đích", route, 164),
        ("date", "Ngày YYYY-MM-DD", proposal.date.to_string(), 10),
        ("note", "Ghi chú", proposal.note.clone(), 1000),
    ];
    let components = fields
        .into_iter()
        .map(|(id, label, value, max)| {
            let style = if id == "note" {
                serenity::InputTextStyle::Paragraph
            } else {
                serenity::InputTextStyle::Short
            };
            let input = serenity::CreateInputText::new(style, label, id)
                .required(true)
                .max_length(max);
            let input = if value.is_empty() {
                input
            } else {
                input.value(value)
            };
            serenity::CreateActionRow::InputText(input)
        })
        .collect();
    serenity::CreateModal::new(
        reference("modal", draft.id, draft.revision),
        "Sửa bản nháp — chưa lưu tiền",
    )
    .components(components)
}
pub(super) fn values(
    data: &serenity::ModalInteractionData,
) -> Result<BTreeMap<String, String>, JournalError> {
    let mut result = BTreeMap::new();
    for row in &data.components {
        for component in &row.components {
            let serenity::ActionRowComponent::InputText(input) = component else {
                return Err(JournalError::InvalidInput);
            };
            let max = match input.custom_id.as_str() {
                "amount" => 32,
                "category" => 80,
                "route" => 164,
                "date" => 10,
                "note" => 1000,
                _ => return Err(JournalError::InvalidInput),
            };
            let value = input.value.clone().ok_or(JournalError::InvalidInput)?;
            if value.encode_utf16().count() > max
                || result.insert(input.custom_id.clone(), value).is_some()
            {
                return Err(JournalError::InvalidInput);
            }
        }
    }
    if result.len() != 5 {
        return Err(JournalError::InvalidInput);
    }
    Ok(result)
}
