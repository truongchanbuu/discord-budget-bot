use super::*;

pub(super) async fn open(store: &PgStore, input: DraftInput<'_>) -> Result<Draft, JournalError> {
    let mut tx = store
        .pool
        .begin()
        .await
        .map_err(|_| JournalError::Storage)?;
    lock_owner(&mut tx, input.owner).await?;
    let row = sqlx::query(
        "SELECT *,trunc(amount)::text AS amount_text FROM fg_drafts WHERE owner=$1 AND origin=$2",
    )
    .bind(input.owner.as_str())
    .bind(input.origin)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| JournalError::Storage)?;
    if let Some(row) = row {
        let draft = mapping::draft(row)?;
        tx.commit().await.map_err(|_| JournalError::Storage)?;
        return Ok(draft);
    }
    let proposal = input.proposal;
    let row = sqlx::query(
        r#"
INSERT INTO fg_drafts(owner, origin, actor, channel, original_note, kind, amount, category, fund,
destination, entry_date, note, expires, created) VALUES($1, $2, $3, $4, $5, $6, $7::text::numeric,
$8, $9, $10, $11, $12, $13, $14) RETURNING *, trunc(amount)::text AS amount_text
"#,
    )
    .bind(input.owner.as_str())
    .bind(input.origin)
    .bind(input.actor)
    .bind(input.channel)
    .bind(&proposal.note)
    .bind(proposal.kind.key())
    .bind(proposal.amount.map(|value| value.whole_dong().to_string()))
    .bind(&proposal.category)
    .bind(&proposal.fund)
    .bind(&proposal.destination)
    .bind(proposal.date)
    .bind(&proposal.note)
    .bind(input.now + chrono::Duration::minutes(15))
    .bind(input.now)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| JournalError::Storage)?;
    let draft = mapping::draft(row)?;
    tx.commit().await.map_err(|_| JournalError::Storage)?;
    Ok(draft)
}
