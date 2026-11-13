use super::{FinanceEvents, send, serenity, views};
use crate::features::ledger::{
    application::ActionResult,
    domain::{JournalError, Month, Receipt, State, local_date},
};
use chrono::Utc;

pub(super) async fn handle(
    handler: &FinanceEvents,
    context: &serenity::Context,
    message: &serenity::Message,
) -> Result<(), JournalError> {
    let actor = match super::actor(&message.author, message.guild_id, &handler.binding) {
        Ok(actor) => actor,
        Err(error) => {
            send(
                message
                    .channel_id
                    .send_message(&context.http, views::message(error.to_string(), vec![])),
            )
            .await?;
            return Ok(());
        }
    };
    let channel = message.channel_id.to_string();
    let input = message.content.trim();
    if let Err(error) = crate::features::ledger::domain::checked_note(input) {
        return respond(context, message, error.to_string()).await;
    }
    if input.encode_utf16().count() > 1000 {
        return respond(context, message, JournalError::InvalidInput.to_string()).await;
    }
    let lower = input.to_lowercase();
    if ["trợ giúp", "tro giup", "help"].contains(&lower.as_str()) {
        return respond(context, message, views::HELP.into()).await;
    }
    if lower == "quỹ" || lower == "quy" {
        let result = handler.journal.funds(handler.scope(&actor, &channel)).await;
        let text = match result {
            Ok(funds) => {
                let mut text = "Các quỹ ảo (tự khai báo):\n".to_string();
                let count = funds.len();
                for (fund, value) in funds.iter().take(12) {
                    text.push_str(&format!(
                        "{}: {}\n",
                        fund.name,
                        views::money(u128::from(value.whole_dong()))
                    ));
                }
                if count > 12 {
                    text.push_str(&format!(
                        "… còn {} quỹ. Tổng kết tháng vẫn tính tất cả quỹ.",
                        count - 12
                    ));
                }
                text
            }
            Err(error) => error.to_string(),
        };
        return respond(context, message, text).await;
    }
    if lower.starts_with("tạo quỹ ") || lower.starts_with("tao quy ") {
        let name = input
            .splitn(3, char::is_whitespace)
            .nth(2)
            .ok_or(JournalError::InvalidInput)?;
        let text = match handler
            .journal
            .create_fund(handler.scope(&actor, &channel), name)
            .await
        {
            Ok(()) => "Đã tạo quỹ chi tiêu với số dư 0 VND.".into(),
            Err(error) => error.to_string(),
        };
        return respond(context, message, text).await;
    }
    if ["tổng kết", "tong ket", "báo cáo", "bao cao"]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        let argument = ["tổng kết", "tong ket", "báo cáo", "bao cao"]
            .iter()
            .find_map(|prefix| lower.strip_prefix(prefix))
            .unwrap_or_default()
            .trim();
        let month = if argument.is_empty() {
            Month::current(local_date(Utc::now()))
        } else {
            Month::parse(argument)
        };
        let text = match month {
            Ok(month) => match handler
                .journal
                .summary(handler.scope(&actor, &channel), month)
                .await
            {
                Ok(report) => {
                    views::summary(&report, month).unwrap_or_else(|error| error.to_string())
                }
                Err(error) => error.to_string(),
            },
            Err(error) => error.to_string(),
        };
        return respond(context, message, text).await;
    }
    let origin = format!("discord:dm:{}", message.id);
    match handler
        .journal
        .preview(handler.scope(&actor, &channel), &origin, input, Utc::now())
        .await
    {
        Ok(draft) => {
            let (text, buttons) = match draft.state {
                State::Posted => views::result(ActionResult::Committed(Receipt {
                    id: draft.entry.ok_or(JournalError::Storage)?,
                    replayed: true,
                })),
                State::Cancelled => views::result(ActionResult::Cancelled),
                State::Pending => (views::preview(&draft), views::buttons(&draft)),
            };
            send(
                message
                    .channel_id
                    .send_message(&context.http, views::message(text, buttons)),
            )
            .await?;
        }
        Err(JournalError::InvalidInput) => {
            respond(
                context,
                message,
                format!(
                    "Chưa hiểu giao dịch hoặc số/ngày chưa hợp lệ.\n{}",
                    views::HELP
                ),
            )
            .await?
        }
        Err(error) => respond(context, message, error.to_string()).await?,
    }
    Ok(())
}
async fn respond(
    context: &serenity::Context,
    message: &serenity::Message,
    text: String,
) -> Result<(), JournalError> {
    send(
        message
            .channel_id
            .send_message(&context.http, views::message(text, vec![])),
    )
    .await?;
    Ok(())
}
