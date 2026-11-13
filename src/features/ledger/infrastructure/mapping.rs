use super::*;
use sqlx::{Row, postgres::PgRow};

pub(super) fn proposal(row: &PgRow) -> Result<Proposal, JournalError> {
    let text = |field| {
        row.try_get::<String, _>(field)
            .map_err(|_| JournalError::Storage)
    };
    let raw = row
        .try_get::<Option<String>, _>("amount_text")
        .map_err(|_| JournalError::Storage)?;
    let amount = raw
        .map(|value| Money::parse("VND", &value).map_err(|_| JournalError::Storage))
        .transpose()?;
    Ok(Proposal {
        kind: Kind::read(&text("kind")?)?,
        amount,
        category: text("category")?,
        fund: text("fund")?,
        destination: row
            .try_get("destination")
            .map_err(|_| JournalError::Storage)?,
        date: row
            .try_get("entry_date")
            .map_err(|_| JournalError::Storage)?,
        note: text("note")?,
    })
}
pub(super) fn draft(row: PgRow) -> Result<Draft, JournalError> {
    let state = match row
        .try_get::<&str, _>("status")
        .map_err(|_| JournalError::Storage)?
    {
        "pending" => State::Pending,
        "posted" => State::Posted,
        "cancelled" => State::Cancelled,
        _ => return Err(JournalError::Storage),
    };
    Ok(Draft {
        id: row.try_get("id").map_err(|_| JournalError::Storage)?,
        owner: OwnerRef::new(row.try_get("owner").map_err(|_| JournalError::Storage)?)
            .map_err(|_| JournalError::Storage)?,
        channel: row.try_get("channel").map_err(|_| JournalError::Storage)?,
        original_note: row
            .try_get("original_note")
            .map_err(|_| JournalError::Storage)?,
        proposal: proposal(&row)?,
        revision: row.try_get("revision").map_err(|_| JournalError::Storage)?,
        expires: row.try_get("expires").map_err(|_| JournalError::Storage)?,
        state,
        entry: row.try_get("entry_id").map_err(|_| JournalError::Storage)?,
    })
}
pub(super) async fn load_draft(
    pool: &PgPool,
    owner: &OwnerRef,
    id: i64,
) -> Result<Draft, JournalError> {
    let row = sqlx::query(
        "SELECT *,trunc(amount)::text AS amount_text FROM fg_drafts WHERE owner=$1 AND id=$2",
    )
    .bind(owner.as_str())
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|_| JournalError::Storage)?
    .ok_or(JournalError::Denied)?;
    draft(row)
}
pub(super) async fn history(
    tx: &mut Transaction<'_, Postgres>,
    owner: &OwnerRef,
) -> Result<(Vec<Fund>, Vec<Entry>), JournalError> {
    let fund_rows = sqlx::query("SELECT name,role FROM fg_funds WHERE owner=$1 ORDER BY name")
        .bind(owner.as_str())
        .fetch_all(&mut **tx)
        .await
        .map_err(|_| JournalError::Storage)?;
    let funds = fund_rows
        .into_iter()
        .map(|row| {
            Ok(Fund {
                name: row.try_get("name").map_err(|_| JournalError::Storage)?,
                role: Role::read(row.try_get("role").map_err(|_| JournalError::Storage)?)?,
            })
        })
        .collect::<Result<_, JournalError>>()?;
    let entry_rows=sqlx::query("SELECT *,trunc(amount)::text AS amount_text FROM fg_entries WHERE owner=$1 ORDER BY entry_date,id").bind(owner.as_str()).fetch_all(&mut **tx).await.map_err(|_|JournalError::Storage)?;
    let entries = entry_rows
        .into_iter()
        .map(|row| {
            Ok(Entry {
                id: row.try_get("id").map_err(|_| JournalError::Storage)?,
                proposal: proposal(&row)?,
            })
        })
        .collect::<Result<_, JournalError>>()?;
    Ok((funds, entries))
}
