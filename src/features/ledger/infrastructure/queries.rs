use super::*;
use chrono::Utc;

pub(super) async fn funds(
    store: &PgStore,
    owner: &OwnerRef,
) -> Result<Vec<(Fund, Money)>, JournalError> {
    let mut tx = store
        .pool
        .begin()
        .await
        .map_err(|_| JournalError::Storage)?;
    lock_owner(&mut tx, owner).await?;
    let (funds, entries) = mapping::history(&mut tx, owner).await?;
    let values = balances(&funds, &entries, local_date(Utc::now()))?;
    let result = funds
        .into_iter()
        .map(|fund| {
            let value = values[&fund.name];
            (fund, value)
        })
        .collect();
    tx.commit().await.map_err(|_| JournalError::Storage)?;
    Ok(result)
}
pub(super) async fn create_fund(
    store: &PgStore,
    owner: &OwnerRef,
    name: String,
) -> Result<(), JournalError> {
    let mut tx = store
        .pool
        .begin()
        .await
        .map_err(|_| JournalError::Storage)?;
    lock_owner(&mut tx, owner).await?;
    let result = sqlx::query(
        "INSERT INTO fg_funds(owner,name,role) VALUES($1,$2,'spending') ON CONFLICT DO NOTHING",
    )
    .bind(owner.as_str())
    .bind(name)
    .execute(&mut *tx)
    .await
    .map_err(|_| JournalError::Storage)?;
    if result.rows_affected() == 0 {
        return Err(JournalError::Duplicate);
    }
    tx.commit().await.map_err(|_| JournalError::Storage)?;
    Ok(())
}
pub(super) async fn summary(
    store: &PgStore,
    owner: &OwnerRef,
    month: Month,
) -> Result<Summary, JournalError> {
    let mut tx = store
        .pool
        .begin()
        .await
        .map_err(|_| JournalError::Storage)?;
    lock_owner(&mut tx, owner).await?;
    let (funds, entries) = mapping::history(&mut tx, owner).await?;
    let report = summarize(&funds, &entries, month, local_date(Utc::now()))?;
    tx.commit().await.map_err(|_| JournalError::Storage)?;
    Ok(report)
}
