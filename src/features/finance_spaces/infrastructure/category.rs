use super::super::domain::{ChannelRef, ServerRef, SpaceError};
use sqlx::PgPool;

pub(crate) enum CategoryReservation {
    Create,
    Existing(ChannelRef),
}

pub(crate) async fn reserve_category(
    pool: &PgPool,
    guild: ServerRef,
) -> Result<CategoryReservation, SpaceError> {
    let reserved = sqlx::query_scalar::<_, String>(
        "INSERT INTO fg_finance_categories(guild_id) VALUES($1) ON CONFLICT DO NOTHING RETURNING guild_id",
    )
    .bind(guild.get().to_string())
    .fetch_optional(pool)
    .await
    .map_err(|_| SpaceError::Storage)?;
    if reserved.is_some() {
        return Ok(CategoryReservation::Create);
    }
    let category: Option<String> =
        sqlx::query_scalar("SELECT category_id FROM fg_finance_categories WHERE guild_id=$1")
            .bind(guild.get().to_string())
            .fetch_one(pool)
            .await
            .map_err(|_| SpaceError::Storage)?;
    let category = category.ok_or(SpaceError::Uncertain)?;
    Ok(CategoryReservation::Existing(ChannelRef::parse(&category)?))
}

pub(crate) async fn bind_category(
    pool: &PgPool,
    guild: ServerRef,
    category: ChannelRef,
) -> Result<(), SpaceError> {
    let changed = sqlx::query(
        "UPDATE fg_finance_categories SET category_id=$2 WHERE guild_id=$1 AND category_id IS NULL",
    )
    .bind(guild.get().to_string())
    .bind(category.get().to_string())
    .execute(pool)
    .await
    .map_err(|_| SpaceError::Storage)?;
    if changed.rows_affected() != 1 {
        return Err(SpaceError::Conflict);
    }
    Ok(())
}

use serenity::{builder::CreateChannel, http::Http, model::prelude::*};

pub(crate) async fn prepare_category(
    pool: &PgPool,
    http: &Http,
    guild: ServerRef,
    configured: Option<ChannelRef>,
) -> Result<ChannelRef, SpaceError> {
    if let Some(category) = configured {
        return Ok(category);
    }
    let bot = http
        .get_current_user()
        .await
        .map_err(|_| SpaceError::Permission)?;
    let guild_id = GuildId::new(guild.get());
    let server = guild_id
        .to_partial_guild(http)
        .await
        .map_err(|_| SpaceError::Permission)?;
    let member = guild_id
        .member(http, bot.id)
        .await
        .map_err(|_| SpaceError::Permission)?;
    super::discord::verify_bot_permissions(super::inspection::role_permissions(&server, &member)?)?;
    match reserve_category(pool, guild).await? {
        CategoryReservation::Existing(category) => Ok(category),
        CategoryReservation::Create => {
            let channel = guild_id
                .create_channel(http, category_definition(guild_id, bot.id))
                .await
                .map_err(category_failure)?;
            let category = ChannelRef::parse(&channel.id.to_string())?;
            bind_category(pool, guild, category).await?;
            Ok(category)
        }
    }
}

fn category_failure(error: serenity::Error) -> SpaceError {
    let (category, http_status, discord_code) = create_diagnostic(&error);
    tracing::warn!(
        event = "space_category_create_failed",
        category,
        http_status,
        discord_code
    );
    SpaceError::Uncertain
}

fn create_diagnostic(error: &serenity::Error) -> (&'static str, Option<u16>, Option<isize>) {
    match error {
        serenity::Error::Http(serenity::http::HttpError::UnsuccessfulRequest(response)) => (
            "rejected",
            Some(response.status_code.as_u16()),
            Some(response.error.code),
        ),
        serenity::Error::Json(_) | serenity::Error::Decode(_, _) => ("decode", None, None),
        serenity::Error::Http(_) => ("transport", None, None),
        _ => ("sdk", None, None),
    }
}

fn category_definition(guild: GuildId, bot: UserId) -> CreateChannel<'static> {
    CreateChannel::new("Tài chính")
        .kind(ChannelType::Category)
        .audit_log_reason("FundGuard managed finance category")
        .permissions([
            PermissionOverwrite {
                kind: PermissionOverwriteType::Role(RoleId::new(guild.get())),
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
            },
            PermissionOverwrite {
                kind: PermissionOverwriteType::Member(bot),
                allow: Permissions::VIEW_CHANNEL
                    | Permissions::SEND_MESSAGES
                    | Permissions::READ_MESSAGE_HISTORY
                    | Permissions::MANAGE_CHANNELS,
                deny: Permissions::empty(),
            },
        ])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn category_diagnostic_excludes_provider_content() {
        let error = serenity::Error::Other("sensitive-provider-marker");
        let diagnostic = create_diagnostic(&error);
        assert!(!format!("{diagnostic:?}").contains("sensitive-provider-marker"));
        assert_eq!(category_failure(error), SpaceError::Uncertain);
    }

    #[test]
    fn managed_category_starts_private() {
        let payload =
            serenity::json::to_value(category_definition(GuildId::new(10), UserId::new(99)))
                .unwrap();
        assert_eq!(payload["name"], "Tài chính");
        assert_eq!(payload["type"], 4);
        assert_eq!(
            payload["permission_overwrites"].as_array().unwrap().len(),
            2
        );
        assert_eq!(
            payload["permission_overwrites"][0]["deny"],
            Permissions::VIEW_CHANNEL.bits().to_string()
        );
        assert_eq!(payload["permission_overwrites"][1]["id"], "99");
        let allowed: u64 = payload["permission_overwrites"][1]["allow"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(allowed & Permissions::MANAGE_ROLES.bits(), 0);
        assert_ne!(allowed & Permissions::VIEW_CHANNEL.bits(), 0);
    }
}
