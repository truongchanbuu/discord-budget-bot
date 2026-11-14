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
    let actor = match handler.request_actor(&message.author, message.guild_id) {
        Ok(actor) => actor,
        Err(error) => {
            if message.guild_id.is_some() {
                return Ok(());
            }
            handler
                .recheck(&message.author, message.guild_id, message.channel_id)
                .await?;
            send(
                message
                    .channel_id
                    .send_message(&context.http, views::message(error.to_string(), vec![])),
            )
            .await?;
            return Ok(());
        }
    };
    if message.guild_id.is_some()
        && handler
            .recheck(&message.author, message.guild_id, message.channel_id)
            .await
            .is_err()
    {
        return Ok(());
    }
    let channel = message.channel_id.to_string();
    let input = if message.guild_id.is_some() {
        let bot = crate::features::finance_spaces::MemberRef::parse(
            &handler
                .bot
                .load(std::sync::atomic::Ordering::Acquire)
                .to_string(),
        )
        .map_err(|_| JournalError::Denied)?;
        crate::features::finance_spaces::application::leading_mention(
            &message.content,
            bot,
            message.mentions.iter().any(|u| u.id.get() == bot.get()),
        )
        .ok_or(JournalError::Denied)?
    } else {
        message.content.trim()
    };
    if let Err(error) = crate::features::ledger::domain::checked_note(input) {
        return respond(handler, context, message, error.to_string()).await;
    }
    if input.encode_utf16().count() > 1000 {
        return respond(
            handler,
            context,
            message,
            JournalError::InvalidInput.to_string(),
        )
        .await;
    }
    let lower = input.to_lowercase();
    if ["trợ giúp", "tro giup", "help"].contains(&lower.as_str()) {
        return respond(handler, context, message, views::HELP.into()).await;
    }
    if lower == "quỹ" || lower == "quy" {
        let result = handler
            .journal
            .funds(handler.scope(&actor, &channel, message.guild_id)?)
            .await;
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
        return respond(handler, context, message, text).await;
    }
    if lower.starts_with("tạo quỹ ") || lower.starts_with("tao quy ") {
        let name = input
            .splitn(3, char::is_whitespace)
            .nth(2)
            .ok_or(JournalError::InvalidInput)?;
        let text = match handler
            .journal
            .create_fund(handler.scope(&actor, &channel, message.guild_id)?, name)
            .await
        {
            Ok(()) => "Đã tạo quỹ chi tiêu với số dư 0 VND.".into(),
            Err(error) => error.to_string(),
        };
        return respond(handler, context, message, text).await;
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
                .summary(handler.scope(&actor, &channel, message.guild_id)?, month)
                .await
            {
                Ok(report) => {
                    views::summary(&report, month).unwrap_or_else(|error| error.to_string())
                }
                Err(error) => error.to_string(),
            },
            Err(error) => error.to_string(),
        };
        return respond(handler, context, message, text).await;
    }
    let origin = format!(
        "discord:{}:{}",
        if message.guild_id.is_some() {
            "guild"
        } else {
            "dm"
        },
        message.id
    );
    match handler
        .journal
        .preview(
            handler.scope(&actor, &channel, message.guild_id)?,
            &origin,
            input,
            Utc::now(),
        )
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
            handler
                .recheck(&message.author, message.guild_id, message.channel_id)
                .await?;
            send(
                message
                    .channel_id
                    .send_message(&context.http, views::message(text, buttons)),
            )
            .await?;
        }
        Err(JournalError::InvalidInput) => {
            respond(
                handler,
                context,
                message,
                format!(
                    "Chưa hiểu giao dịch hoặc số/ngày chưa hợp lệ.\n{}",
                    views::HELP
                ),
            )
            .await?
        }
        Err(error) => respond(handler, context, message, error.to_string()).await?,
    }
    Ok(())
}
async fn respond(
    handler: &FinanceEvents,
    context: &serenity::Context,
    message: &serenity::Message,
    text: String,
) -> Result<(), JournalError> {
    handler
        .recheck(&message.author, message.guild_id, message.channel_id)
        .await?;
    send(
        message
            .channel_id
            .send_message(&context.http, views::message(text, vec![])),
    )
    .await?;
    Ok(())
}
