mod port;
use super::domain::{
    Draft, Fund, JournalError, Month, Proposal, State, Summary, local_date, name, parse,
};
use crate::features::{
    financial_core::{Money, OwnerRef},
    identity::{OwnerBinding, TrustedActor, resolve_owner},
};
use chrono::{DateTime, Utc};
pub(crate) use port::*;
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub(crate) struct Journal {
    store: Arc<dyn JournalStore>,
    spaces: Option<crate::features::finance_spaces::application::Spaces>,
}
pub(crate) struct Scope<'a> {
    pub actor: &'a TrustedActor,
    pub binding: &'a OwnerBinding,
    pub channel: &'a str,
    pub guild: Option<(
        crate::features::finance_spaces::ServerRef,
        crate::features::finance_spaces::MemberRef,
    )>,
}

fn authorized<'a>(scope: &Scope<'a>) -> Result<&'a OwnerRef, JournalError> {
    resolve_owner(Some(scope.actor), Some(scope.binding)).map_err(|_| JournalError::Denied)
}
async fn bounded<T>(
    future: impl std::future::Future<Output = Result<T, JournalError>>,
) -> Result<T, JournalError> {
    tokio::time::timeout(Duration::from_secs(5), future)
        .await
        .map_err(|_| JournalError::Timeout)?
}
impl Journal {
    pub(crate) fn new(store: Arc<dyn JournalStore>) -> Self {
        Self {
            store,
            spaces: None,
        }
    }
    pub(crate) fn with_spaces(
        mut self,
        spaces: crate::features::finance_spaces::application::Spaces,
    ) -> Self {
        self.spaces = Some(spaces);
        self
    }
    pub(crate) async fn preview(
        &self,
        scope: Scope<'_>,
        origin: &str,
        input: &str,
        now: DateTime<Utc>,
    ) -> Result<Draft, JournalError> {
        let owner = resolve_scope(&scope, self.spaces.as_ref()).await?;
        if owner.as_str().len() > 512
            || origin.is_empty()
            || origin.len() > 128
            || scope.channel.is_empty()
            || scope.channel.len() > 32
        {
            return Err(JournalError::InvalidInput);
        }
        let proposal = parse(input, local_date(now))?;
        bounded(self.store.open_draft(DraftInput {
            owner: &owner,
            actor: scope.actor.account().as_u64().to_string(),
            channel: scope.channel,
            origin,
            proposal,
            now,
        }))
        .await
    }
    pub(crate) async fn draft(
        &self,
        scope: Scope<'_>,
        id: i64,
        revision: i32,
    ) -> Result<Draft, JournalError> {
        let owner = resolve_scope(&scope, self.spaces.as_ref()).await?;
        let draft = bounded(self.store.draft(&owner, id)).await?;
        if draft.owner != owner {
            return Err(JournalError::Denied);
        }
        if draft.channel != scope.channel {
            return Err(JournalError::Denied);
        }
        if draft.revision != revision {
            return Err(JournalError::Stale);
        }
        if draft.state == State::Cancelled {
            return Err(JournalError::Cancelled);
        }
        if draft.state == State::Pending && draft.expires <= Utc::now() {
            return Err(JournalError::Expired);
        }
        Ok(draft)
    }
    pub(crate) async fn act(
        &self,
        scope: Scope<'_>,
        id: i64,
        revision: i32,
        action: Action,
    ) -> Result<ActionResult, JournalError> {
        let owner = resolve_scope(&scope, self.spaces.as_ref()).await?;
        bounded(self.store.act(
            &owner,
            Mutation {
                id,
                revision,
                channel: scope.channel.into(),
                actor: scope.actor.account().as_u64().to_string(),
                action,
            },
        ))
        .await
    }
    pub(crate) async fn funds(&self, scope: Scope<'_>) -> Result<Vec<(Fund, Money)>, JournalError> {
        let owner = resolve_scope(&scope, self.spaces.as_ref()).await?;
        bounded(self.store.funds(&owner)).await
    }
    pub(crate) async fn create_fund(
        &self,
        scope: Scope<'_>,
        value: &str,
    ) -> Result<(), JournalError> {
        let owner = resolve_scope(&scope, self.spaces.as_ref()).await?;
        bounded(self.store.create_fund(&owner, name(value)?)).await
    }
    pub(crate) async fn summary(
        &self,
        scope: Scope<'_>,
        month: Month,
    ) -> Result<Summary, JournalError> {
        let owner = resolve_scope(&scope, self.spaces.as_ref()).await?;
        bounded(self.store.summary(&owner, month)).await
    }
}

pub(crate) fn edit(
    mut proposal: Proposal,
    values: &std::collections::BTreeMap<String, String>,
) -> Result<Proposal, JournalError> {
    if values.len() != 5
        || ["amount", "category", "route", "date", "note"]
            .iter()
            .any(|key| !values.contains_key(*key))
    {
        return Err(JournalError::InvalidInput);
    }
    proposal.amount = Some(super::domain::amount(&values["amount"])?);
    proposal.category = values["category"].trim().to_owned();
    let route: Vec<_> = values["route"].split("->").collect();
    if route.len() > 2 {
        return Err(JournalError::InvalidInput);
    }
    proposal.fund = name(route[0])?;
    proposal.destination = route.get(1).map(|value| name(value)).transpose()?;
    proposal.date = super::domain::date(&values["date"])?;
    proposal.note = values["note"].clone();
    proposal.validate(local_date(Utc::now()))?;
    Ok(proposal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::identity::{BindingState, DiscordAccountId};
    #[test]
    fn current_owner_policy_is_required() {
        let actor = TrustedActor::new(DiscordAccountId::parse("123").unwrap());
        let binding = OwnerBinding::new(
            DiscordAccountId::parse("124").unwrap(),
            OwnerRef::new("synthetic-owner".into()).unwrap(),
            BindingState::Enabled,
        );
        assert_eq!(
            authorized(&Scope {
                actor: &actor,
                binding: &binding,
                channel: "1",
                guild: None
            })
            .unwrap_err(),
            JournalError::Denied
        );
    }
}

#[cfg(test)]
mod scope_tests;

async fn resolve_scope(
    scope: &Scope<'_>,
    spaces: Option<&crate::features::finance_spaces::application::Spaces>,
) -> Result<OwnerRef, JournalError> {
    let Some((guild, bot)) = scope.guild else {
        return authorized(scope).cloned();
    };
    let spaces = spaces.ok_or(JournalError::Denied)?;
    if guild != spaces.settings.guild {
        return Err(JournalError::Denied);
    }
    let member = crate::features::finance_spaces::MemberRef::parse(
        &scope.actor.account().as_u64().to_string(),
    )
    .map_err(|_| JournalError::Denied)?;
    let channel = crate::features::finance_spaces::ChannelRef::parse(scope.channel)
        .map_err(|_| JournalError::Denied)?;
    let access = spaces
        .authorize_finance(member, channel, bot)
        .await
        .map_err(|_| JournalError::Denied)?;
    resolve_owner(Some(scope.actor), Some(&access.binding))
        .cloned()
        .map_err(|_| JournalError::Denied)
}
