use super::views;
use super::*;
use serenity::builder::*;
pub(super) fn button(
    action: &str,
    id: impl std::fmt::Display,
    revision: i64,
    label: &str,
) -> CreateButton {
    CreateButton::new(format!("fgspace:{action}:{id}:{revision}"))
        .label(label)
        .style(ButtonStyle::Secondary)
}
pub(super) use super::references::parse;
pub(super) async fn handle(
    handler: &SpacesEvents,
    context: &Context,
    interaction: &ComponentInteraction,
) -> Result<(), SpaceError> {
    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        interaction.defer_ephemeral(&context.http),
    )
    .await
    .map_err(|_| SpaceError::Timeout)?
    .map_err(|_| SpaceError::Permission)?;
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(20),
        action(handler, interaction),
    )
    .await;
    let (mut text,rows)=match result {
        Ok(Ok(value))=>value,
        _=>("Không thể xác nhận yêu cầu. Quyền chưa được xác minh có thể vẫn đang chờ; không thử tạo kênh mới để bỏ qua trạng thái này.".into(),vec![]),
    };
    if !text.contains(views::PRIVACY) {
        text.push_str(&format!("\n{}", views::PRIVACY));
    }
    let response = EditInteractionResponse::new()
        .content(text)
        .components(rows)
        .allowed_mentions(CreateAllowedMentions::new())
        .flags(MessageFlags::SUPPRESS_EMBEDS);
    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        interaction.edit_response(&context.http, response),
    )
    .await
    .map_err(|_| SpaceError::Timeout)?
    .map_err(|_| SpaceError::Permission)?;
    Ok(())
}
pub(super) async fn action(
    handler: &SpacesEvents,
    value: &ComponentInteraction,
) -> Result<(String, Vec<CreateActionRow>), SpaceError> {
    if !handler.spaces.connected.load(Ordering::Acquire) {
        return Err(SpaceError::Pending);
    }
    if value.guild_id.map(|id| id.get()) != Some(handler.spaces.settings.guild.get())
        || value.user.bot
    {
        return Err(SpaceError::Denied);
    }
    let member = MemberRef::parse(&value.user.id.to_string())?;
    handler.spaces.admit_command(member)?;
    let channel = ChannelRef::parse(&value.channel_id.to_string())?;
    let bot = MemberRef::parse(&handler.bot.load(Ordering::Acquire).to_string())?;
    let r = parse(&value.data.custom_id)?;
    if r.action == "confirm" {
        handler
            .spaces
            .confirm_viewer(member, channel, r.id.into(), r.revision, bot)
            .await?;
        return Ok((
            format!("Đã xác minh thay đổi quyền. {}", views::PRIVACY),
            vec![],
        ));
    }
    if r.action == "cancel" {
        handler
            .spaces
            .cancel_operation(member, channel, r.id.into(), r.revision)
            .await?;
        return Ok((
            "Đã hủy xác nhận chưa dispatch; không thay đổi quyền Discord.".into(),
            vec![],
        ));
    }
    let id = SpaceId::new(r.id.parse().map_err(|_| SpaceError::Invalid)?)?;
    if r.action == "recover" {
        let ready = handler
            .spaces
            .confirm_recovery(member, channel, id, r.revision, bot)
            .await?;
        return Ok((
            format!(
                "Đã phục hồi: <#{}>. {}",
                ready.channel.ok_or(SpaceError::Pending)?.get(),
                views::PRIVACY
            ),
            vec![],
        ));
    }
    let access = handler
        .spaces
        .authorize_owner_management(member, channel, id)
        .await?;
    if access.revision != r.revision {
        return Err(SpaceError::Conflict);
    }
    let grants = handler.spaces.list_grants(member, channel, id).await?;
    if r.action == "manage" {
        let mut text = format!("Quản lý người xem. {}", views::PRIVACY);
        for grant in grants {
            text.push_str(&format!(
                "\nID {}: {:?} · {}",
                grant.member.get(),
                grant.confirmed,
                grant.changed_at.to_rfc3339()
            ));
        }
        return Ok((
            text,
            vec![CreateActionRow::Buttons(vec![
                button("add", id.get(), access.revision, "Thêm người xem"),
                button("revoke", id.get(), access.revision, "Thu hồi"),
            ])],
        ));
    }
    if r.action == "add" {
        let menu = CreateSelectMenu::new(
            format!("fgspace:select_add:{}:{}", id.get(), access.revision),
            CreateSelectMenuKind::User {
                default_users: None,
            },
        )
        .min_values(1)
        .max_values(1);
        return Ok((
            "Chọn đúng một thành viên để xem lịch sử và các bài đăng sau này.".into(),
            vec![CreateActionRow::SelectMenu(menu)],
        ));
    }
    if r.action == "revoke" {
        let options: Vec<_> = grants
            .iter()
            .filter(|g| g.confirmed != ViewerConfirmed::Absent)
            .take(8)
            .map(|g| {
                CreateSelectMenuOption::new(
                    format!("Người xem {}", g.member.get()),
                    g.member.get().to_string(),
                )
            })
            .collect();
        if options.is_empty() {
            return Ok(("Không có người xem cần thu hồi.".into(), vec![]));
        }
        return Ok((
            "Chọn quyền cần thu hồi.".into(),
            vec![CreateActionRow::SelectMenu(
                CreateSelectMenu::new(
                    format!("fgspace:select_revoke:{}:{}", id.get(), access.revision),
                    CreateSelectMenuKind::String { options },
                )
                .min_values(1)
                .max_values(1),
            )],
        ));
    }
    let (target, kind) = match (&value.data.kind, r.action) {
        (ComponentInteractionDataKind::UserSelect { values }, "select_add")
            if values.len() == 1 =>
        {
            (
                MemberRef::parse(&values[0].to_string())?,
                OperationKind::GrantViewer,
            )
        }
        (ComponentInteractionDataKind::StringSelect { values }, "select_revoke")
            if values.len() == 1 =>
        {
            (MemberRef::parse(&values[0])?, OperationKind::RevokeViewer)
        }
        _ => return Err(SpaceError::Invalid),
    };
    let op = handler
        .spaces
        .prepare_viewer(
            ViewerRequest {
                member,
                channel,
                space: id,
                target,
                kind,
                source: format!("interaction:{}", value.id),
            },
            bot,
        )
        .await?;
    Ok((
        format!(
            "Xác nhận thay đổi quyền cho ID {} trong 5 phút. {}",
            target.get(),
            views::PRIVACY
        ),
        vec![CreateActionRow::Buttons(vec![
            button("confirm", &op.id, op.expected_revision, "Xác nhận"),
            button("cancel", &op.id, op.expected_revision, "Hủy"),
        ])],
    ))
}
