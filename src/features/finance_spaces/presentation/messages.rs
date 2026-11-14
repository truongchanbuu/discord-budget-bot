use super::*;
pub(super) async fn handle(
    handler: &SpacesEvents,
    context: &Context,
    message: &Message,
    bot: MemberRef,
) -> Result<(), SpaceError> {
    if message.author.bot
        || message.webhook_id.is_some()
        || message.kind != MessageType::Regular
        || !message.attachments.is_empty()
        || !message.sticker_items.is_empty()
    {
        return Ok(());
    }
    let Some(input) = leading_mention(
        &message.content,
        bot,
        message
            .mentions
            .iter()
            .any(|user| user.id.get() == bot.get()),
    ) else {
        return Ok(());
    };
    if classify(input) == Route::Finance {
        return Ok(());
    }
    let channel = ChannelRef::parse(&message.channel_id.to_string())?;
    let member = MemberRef::parse(&message.author.id.to_string())?;
    handler.spaces.admit_command(member)?;
    handler.wake.notify_one();
    if classify(input) == Route::Start && handler.spaces.settings.permits_onboarding(channel) {
        let current = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            message.channel_id.to_channel(&context.http),
        )
        .await
        .map_err(|_| SpaceError::Timeout)?
        .map_err(|_| SpaceError::Permission)?
        .guild()
        .ok_or(SpaceError::Denied)?;
        if current.guild_id.get() != handler.spaces.settings.guild.get()
            || current.kind != ChannelType::Text
        {
            return Err(SpaceError::Denied);
        }
        let outcome = tokio::time::timeout(std::time::Duration::from_secs(20), async {
            match handler
                .spaces
                .onboard(
                    member,
                    channel,
                    bot,
                    &format!("discord:guild:{}", message.id),
                )
                .await
            {
                Ok(space) => Ok(Opening::Ready(space)),
                Err(_) => handler
                    .spaces
                    .request_recovery(member, channel, &format!("recovery:{}", message.id))
                    .await
                    .map(Opening::Recovery),
            }
        })
        .await;
        let mut rows = vec![];
        let text = match outcome {
            Ok(Ok(Opening::Ready(space))) => {
                let id = space.channel.ok_or(SpaceError::Pending)?;
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(2),
                    ChannelId::new(id.get())
                        .send_message(&context.http, views::message(views::welcome())),
                )
                .await;
                format!("Kênh của bạn: <#{}>. {}", id.get(), views::PRIVACY)
            }
            Ok(Ok(Opening::Recovery(space))) => {
                rows = vec![serenity::builder::CreateActionRow::Buttons(vec![
                    super::components::button(
                        "recover",
                        space.id.get(),
                        space.revision,
                        "Xác nhận phục hồi",
                    ),
                ])];
                format!(
                    "Phục hồi tạo kênh mới và giữ journal; không mang người xem cũ sang. Quyền/lịch sử của kênh cũ có thể chưa dọn xong. {}",
                    views::PRIVACY
                )
            }
            _ => format!(
                "Yêu cầu đang chờ hoặc chưa thể xác minh quyền. Không thử tạo thêm kênh để bỏ qua trạng thái này. {}",
                views::PRIVACY
            ),
        };
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            message
                .channel_id
                .send_message(&context.http, views::message(text).components(rows)),
        )
        .await
        .map_err(|_| SpaceError::Timeout)?
        .map_err(|_| SpaceError::Permission)?;
    }
    if classify(input) == Route::Access
        && let Ok(access) = handler.spaces.management_status(member, channel).await
    {
        let launcher = views::message(format!(
            "Mở quản lý quyền riêng của bạn. {}",
            views::PRIVACY
        ))
        .components(vec![serenity::builder::CreateActionRow::Buttons(vec![
            super::components::button(
                "manage",
                access.space.get(),
                access.revision,
                "Quản lý người xem",
            ),
        ])]);
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            message.channel_id.send_message(&context.http, launcher),
        )
        .await
        .map_err(|_| SpaceError::Timeout)?
        .map_err(|_| SpaceError::Permission)?;
    }
    Ok(())
}

enum Opening {
    Ready(FinanceSpace),
    Recovery(FinanceSpace),
}
