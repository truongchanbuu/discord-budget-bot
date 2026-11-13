use super::{reference, serenity};
use crate::features::ledger::{
    application::ActionResult,
    domain::{Draft, Kind, State, Summary},
};

pub(super) const HELP: &str = "Nhắn riêng: nhận lương 15tr; chi 100k ăn uống; donate 100k cho vtuber; tiết kiệm 3tr; đầu tư 2tr; số dư đầu kỳ 2tr.\nLệnh: quỹ; tạo quỹ Du lịch; tổng kết YYYY-MM; trợ giúp.\nBấm Lưu sau khi kiểm tra. Thiếu/sai số tiền dùng Sửa. Chỉ ghi nhận tiền đã nhận/đã chi; đây là sổ ảo, không xác minh ngân hàng.";
pub(super) fn money(value: u128) -> String {
    let digits = value.to_string();
    let mut result = String::new();
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            result.push('.');
        }
        result.push(character);
    }
    result.push_str(" VND");
    result
}
pub(super) fn preview(draft: &Draft) -> String {
    let proposal = &draft.proposal;
    let amount = proposal
        .amount
        .map(|value| money(u128::from(value.whole_dong())))
        .unwrap_or_else(|| "Chưa rõ — bấm Sửa để nhập".into());
    let kind = match proposal.kind {
        Kind::Income => "Thu đã nhận",
        Kind::Expense => "Chi tiêu",
        Kind::Transfer => "Chuyển quỹ",
        Kind::Opening => "Số dư khởi tạo",
    };
    let route = proposal
        .destination
        .as_ref()
        .map(|target| format!("{} → {target}", proposal.fund))
        .unwrap_or_else(|| {
            if proposal.kind == Kind::Transfer {
                format!("{} -> chưa chọn quỹ đích; bấm Sửa", proposal.fund)
            } else {
                proposal.fund.clone()
            }
        });
    let changed = if draft.original_note != proposal.note {
        "\nGhi chú đã sửa; nguồn gốc vẫn được giữ trong sổ."
    } else {
        ""
    };
    format!(
        "Bản nháp #{} (phiên bản {})\nLoại: {kind}\nSố tiền: {amount}\nDanh mục: {}\nQuỹ: {route}\nNgày: {}\nGhi chú: {}\nTự khai báo / số dư ảo; chưa xác minh ngân hàng. Kiểm tra rồi Lưu; hết hạn sau 15 phút.{changed}",
        draft.id, draft.revision, proposal.category, proposal.date, proposal.note
    )
}
pub(super) fn buttons(draft: &Draft) -> Vec<serenity::CreateActionRow> {
    if draft.state != State::Pending {
        return vec![];
    }
    vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(reference("save", draft.id, draft.revision))
            .label("Lưu")
            .style(serenity::ButtonStyle::Success),
        serenity::CreateButton::new(reference("edit", draft.id, draft.revision)).label("Sửa"),
        serenity::CreateButton::new(reference("cancel", draft.id, draft.revision))
            .label("Hủy")
            .style(serenity::ButtonStyle::Danger),
    ])]
}
pub(super) fn message(
    content: String,
    components: Vec<serenity::CreateActionRow>,
) -> serenity::CreateMessage {
    serenity::CreateMessage::new()
        .content(content)
        .components(components)
        .allowed_mentions(serenity::CreateAllowedMentions::new())
        .flags(serenity::MessageFlags::SUPPRESS_EMBEDS)
}
pub(super) fn edited(
    content: String,
    components: Vec<serenity::CreateActionRow>,
) -> serenity::EditInteractionResponse {
    serenity::EditInteractionResponse::new()
        .content(content)
        .components(components)
        .allowed_mentions(serenity::CreateAllowedMentions::new())
        .flags(serenity::MessageFlags::SUPPRESS_EMBEDS)
}
pub(super) fn private_error(content: String) -> serenity::CreateInteractionResponse {
    serenity::CreateInteractionResponse::Message(
        serenity::CreateInteractionResponseMessage::new()
            .content(content)
            .ephemeral(true)
            .allowed_mentions(serenity::CreateAllowedMentions::new())
            .flags(
                serenity::InteractionResponseFlags::EPHEMERAL
                    | serenity::InteractionResponseFlags::SUPPRESS_EMBEDS,
            ),
    )
}
pub(super) fn result(output: ActionResult) -> (String, Vec<serenity::CreateActionRow>) {
    match output {
        ActionResult::Preview(draft) => (preview(&draft), buttons(&draft)),
        ActionResult::Cancelled => ("Đã hủy bản nháp. Không thay đổi tiền.".into(), vec![]),
        ActionResult::Committed(receipt) => (
            format!(
                "{} bút toán #{}. Tự xác nhận, chưa xác minh ngân hàng.",
                if receipt.replayed {
                    "Đã lưu trước đó"
                } else {
                    "Đã lưu"
                },
                receipt.id
            ),
            vec![],
        ),
    }
}
pub(super) fn summary(
    report: &Summary,
    month: super::super::domain::Month,
) -> Result<String, super::JournalError> {
    let percentage = report
        .percent()?
        .map(|value| format!("{value}% (lấy số nguyên)"))
        .unwrap_or_else(|| "Không áp dụng".into());
    let mut text = format!(
        "Tổng kết {} — sổ ảo / tự khai báo\nĐầu kỳ/khởi tạo: {}\nThu đã nhận: {}\nĐã chi: {}\nChuyển vào tiết kiệm: {}\nChuyển vào đầu tư: {}\nTổng cuối kỳ: {}\nCòn ở quỹ chi tiêu: {}\nChi/thu: {percentage}\nChi theo danh mục:\n",
        month.first.format("%Y-%m"),
        money(report.opening),
        money(report.income),
        money(report.expenses),
        money(report.saving),
        money(report.investment),
        money(report.closing),
        money(report.available)
    );
    let mut categories: Vec<_> = report.categories.iter().collect();
    categories.sort_by_key(|(_, value)| std::cmp::Reverse(**value));
    for (category, value) in categories.iter().take(10) {
        text.push_str(&format!("{category}: {}\n", money(**value)));
    }
    if categories.len() > 10 {
        text.push_str(&format!(
            "… còn {} danh mục; tổng Đã chi ở trên bao gồm tất cả.",
            categories.len() - 10
        ));
    }
    Ok(text)
}
