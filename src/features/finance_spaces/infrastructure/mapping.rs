use super::super::domain::*;
use super::incarnation;
use crate::features::financial_core::OwnerRef;
use sqlx::{Row, postgres::PgRow};

pub(super) fn space(row: PgRow) -> Result<FinanceSpace, SpaceError> {
    let text = |name| {
        row.try_get::<String, _>(name)
            .map_err(|_| SpaceError::Storage)
    };
    let channel: Option<String> = row.try_get("channel_id").map_err(|_| SpaceError::Storage)?;
    let value = FinanceSpace {
        id: SpaceId::new(row.try_get("id").map_err(|_| SpaceError::Storage)?)?,
        guild: ServerRef::parse(&text("guild_id")?)?,
        member: MemberRef::parse(&text("member_id")?)?,
        journal_owner: OwnerRef::new(text("journal_owner")?).map_err(|_| SpaceError::Storage)?,
        legacy: row
            .try_get("legacy_binding")
            .map_err(|_| SpaceError::Storage)?,
        owner_joined_at: row
            .try_get::<Option<String>, _>("owner_joined_at")
            .map_err(|_| SpaceError::Storage)?
            .as_deref()
            .map(incarnation::decode)
            .transpose()?,
        generation: text("generation")?,
        channel: channel.as_deref().map(ChannelRef::parse).transpose()?,
        category: ChannelRef::parse(&text("category_id")?)?,
        state: SpaceState::parse(&text("state")?)?,
        revision: row.try_get("revision").map_err(|_| SpaceError::Storage)?,
        current_operation: row
            .try_get("current_operation_id")
            .map_err(|_| SpaceError::Storage)?,
    };
    if value.revision < 0
        || !valid_token(&value.generation)
        || value
            .current_operation
            .as_ref()
            .is_some_and(|id| !valid_token(id))
        || (value.state == SpaceState::Ready && !value.is_bound_ready())
    {
        return Err(SpaceError::Storage);
    }
    Ok(value)
}

pub(super) fn operation(row: PgRow) -> Result<SpaceOperation, SpaceError> {
    let text = |name| {
        row.try_get::<String, _>(name)
            .map_err(|_| SpaceError::Storage)
    };
    let target: Option<String> = row
        .try_get("target_member")
        .map_err(|_| SpaceError::Storage)?;
    let channel: Option<String> = row
        .try_get("returned_channel_id")
        .map_err(|_| SpaceError::Storage)?;
    let desired: Option<String> = row
        .try_get("desired_access")
        .map_err(|_| SpaceError::Storage)?;
    let outcome: Option<String> = row.try_get("outcome").map_err(|_| SpaceError::Storage)?;
    let value = SpaceOperation {
        id: text("id")?,
        space: SpaceId::new(row.try_get("space_id").map_err(|_| SpaceError::Storage)?)?,
        kind: OperationKind::parse(&text("kind")?)?,
        source: text("source")?,
        actor: MemberRef::parse(&text("actor")?)?,
        generation: text("generation")?,
        expected_revision: row
            .try_get("expected_revision")
            .map_err(|_| SpaceError::Storage)?,
        target: target.as_deref().map(MemberRef::parse).transpose()?,
        target_joined_at: row
            .try_get::<Option<String>, _>("target_joined_at")
            .map_err(|_| SpaceError::Storage)?
            .as_deref()
            .map(incarnation::decode)
            .transpose()?,
        phase: OperationPhase::parse(&text("phase")?)?,
        returned_channel: channel.as_deref().map(ChannelRef::parse).transpose()?,
        created_at: row.try_get("created_at").map_err(|_| SpaceError::Storage)?,
        dispatched_at: row
            .try_get("dispatched_at")
            .map_err(|_| SpaceError::Storage)?,
        completed_at: row
            .try_get("completed_at")
            .map_err(|_| SpaceError::Storage)?,
        outcome: outcome
            .as_deref()
            .map(OperationOutcome::parse)
            .transpose()?,
        desired_access: match desired.as_deref() {
            None => None,
            Some("viewer") => Some(DesiredAccess::Viewer),
            Some("absent") => Some(DesiredAccess::Absent),
            _ => return Err(SpaceError::Storage),
        },
    };
    value.validate().map_err(|_| SpaceError::Storage)?;
    Ok(value)
}

pub(super) fn viewer(row: PgRow) -> Result<ViewerGrant, SpaceError> {
    let text = |name| {
        row.try_get::<String, _>(name)
            .map_err(|_| SpaceError::Storage)
    };
    Ok(ViewerGrant {
        space: SpaceId::new(row.try_get("space_id").map_err(|_| SpaceError::Storage)?)?,
        generation: text("generation")?,
        member: MemberRef::parse(&text("member_id")?)?,
        joined_at: incarnation::decode(&text("member_joined_at")?)?,
        desired: match text("desired_state")?.as_str() {
            "viewer" => ViewerDesired::Viewer,
            "revoked" => ViewerDesired::Revoked,
            _ => return Err(SpaceError::Storage),
        },
        confirmed: match text("confirmed_state")?.as_str() {
            "pending" => ViewerConfirmed::Pending,
            "viewer" => ViewerConfirmed::Viewer,
            "absent" => ViewerConfirmed::Absent,
            "uncertain" => ViewerConfirmed::Uncertain,
            _ => return Err(SpaceError::Storage),
        },
        revision: row.try_get("revision").map_err(|_| SpaceError::Storage)?,
        operation: text("operation_id")?,
        granted_by: MemberRef::parse(&text("granted_by")?)?,
        changed_at: row.try_get("changed_at").map_err(|_| SpaceError::Storage)?,
    })
}
