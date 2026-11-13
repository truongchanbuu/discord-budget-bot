use super::*;
use chrono::Utc;
use sqlx::Row;

pub(super) async fn apply(
    store: &PgStore,
    owner: &OwnerRef,
    mutation: Mutation,
) -> Result<ActionResult, JournalError> {
    let mut tx = store
        .pool
        .begin()
        .await
        .map_err(|_| JournalError::Storage)?;
    lock_owner(&mut tx, owner).await?;
    let row = sqlx::query(
        "SELECT *,trunc(amount)::text AS amount_text FROM fg_drafts WHERE owner=$1 AND id=$2 FOR UPDATE",
    )
    .bind(owner.as_str())
    .bind(mutation.id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| JournalError::Storage)?
    .ok_or(JournalError::Denied)?;
    let draft = mapping::draft(row)?;
    if draft.channel != mutation.channel {
        return Err(JournalError::Denied);
    }
    if draft.state == State::Posted {
        if !matches!(mutation.action, Action::Confirm) {
            return Err(JournalError::Stale);
        }
        let id = draft.entry.ok_or(JournalError::Storage)?;
        tx.commit().await.map_err(|_| JournalError::Storage)?;
        return Ok(ActionResult::Committed(Receipt { id, replayed: true }));
    }
    if draft.state == State::Cancelled {
        return Err(JournalError::Cancelled);
    }
    if draft.revision != mutation.revision {
        return Err(JournalError::Stale);
    }
    let now = Utc::now();
    if draft.expires <= now {
        return Err(JournalError::Expired);
    }
    let output = match mutation.action {
        Action::Cancel => {
            sqlx::query("UPDATE fg_drafts SET status='cancelled' WHERE id=$1 AND owner=$2")
                .bind(draft.id)
                .bind(owner.as_str())
                .execute(&mut *tx)
                .await
                .map_err(|_| JournalError::Storage)?;
            ActionResult::Cancelled
        }
        Action::Edit(proposal) => {
            if proposal.kind != draft.proposal.kind {
                return Err(JournalError::InvalidInput);
            }
            proposal.validate(local_date(now))?;
            let revision = draft.revision.checked_add(1).ok_or(JournalError::Stale)?;
            let row = sqlx::query(
                r#"
UPDATE fg_drafts SET amount=$1::text::numeric, category=$2, fund=$3, destination=$4, entry_date=$5,
note=$6, revision=$7 WHERE id=$8 AND owner=$9 RETURNING *, trunc(amount)::text AS amount_text
"#,
            )
            .bind(proposal.amount.map(|value| value.whole_dong().to_string()))
            .bind(proposal.category)
            .bind(proposal.fund)
            .bind(proposal.destination)
            .bind(proposal.date)
            .bind(proposal.note)
            .bind(revision)
            .bind(draft.id)
            .bind(owner.as_str())
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| JournalError::Storage)?;
            ActionResult::Preview(Box::new(mapping::draft(row)?))
        }
        Action::Confirm => {
            let (funds, mut entries) = mapping::history(&mut tx, owner).await?;
            entries.push(Entry {
                id: i64::MAX,
                proposal: draft.proposal.clone(),
            });
            balances(&funds, &entries, local_date(now))?;
            let amount = draft.proposal.validate(local_date(now))?;
            let row = sqlx::query(
                r#"
INSERT INTO fg_entries(owner, draft_id, origin, actor, kind, amount, category, fund, destination,
entry_date, note, created, confirmed, confirmed_actor) SELECT owner, id, origin, actor, kind,
$1::text::numeric, category, fund, destination, entry_date, note, created, $2, $5 FROM fg_drafts
WHERE owner=$3 AND id=$4 RETURNING id
"#,
            )
            .bind(amount.whole_dong().to_string())
            .bind(now)
            .bind(owner.as_str())
            .bind(draft.id)
            .bind(&mutation.actor)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| JournalError::Storage)?;
            let id = row.try_get("id").map_err(|_| JournalError::Storage)?;
            sqlx::query(
                "UPDATE fg_drafts SET status='posted',entry_id=$1 WHERE owner=$2 AND id=$3",
            )
            .bind(id)
            .bind(owner.as_str())
            .bind(draft.id)
            .execute(&mut *tx)
            .await
            .map_err(|_| JournalError::Storage)?;
            ActionResult::Committed(Receipt {
                id,
                replayed: false,
            })
        }
    };
    tx.commit().await.map_err(|_| JournalError::Storage)?;
    Ok(output)
}
