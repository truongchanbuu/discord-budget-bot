use super::super::domain::*;
use crate::features::{
    financial_core::OwnerRef,
    identity::{DiscordAccountId, OwnerBinding, TrustedActor, resolve_owner},
};

pub(crate) fn legacy_owner(
    member: MemberRef,
    binding: &OwnerBinding,
) -> Result<Option<&OwnerRef>, SpaceError> {
    if member.get() != binding.configured_account().as_u64() {
        return Ok(None);
    }
    let account =
        DiscordAccountId::parse(&member.get().to_string()).map_err(|_| SpaceError::Denied)?;
    resolve_owner(Some(&TrustedActor::new(account)), Some(binding))
        .map(Some)
        .map_err(|_| SpaceError::Denied)
}
pub(super) fn verify_legacy(
    space: &FinanceSpace,
    member: MemberRef,
    binding: &OwnerBinding,
) -> Result<(), SpaceError> {
    let expected = legacy_owner(member, binding)?;
    if space.legacy != expected.is_some()
        || expected.is_some_and(|owner| owner != &space.journal_owner)
    {
        return Err(SpaceError::Denied);
    }
    Ok(())
}

use super::{SpaceTransition, Spaces, bounded};
pub(crate) struct FinancialAccess {
    pub(crate) binding: OwnerBinding,
}
pub(crate) struct OwnerManagementAccess {
    pub(crate) space: SpaceId,
    pub(crate) revision: i64,
    pub(crate) generation: String,
    pub(crate) incarnation: chrono::DateTime<chrono::Utc>,
}
impl OwnerManagementAccess {
    pub(super) fn check_current(&self, space: &FinanceSpace) -> Result<(), SpaceError> {
        if self.space != space.id
            || self.revision != space.revision
            || self.generation != space.generation
            || Some(self.incarnation) != space.owner_joined_at
        {
            return Err(SpaceError::Conflict);
        }
        Ok(())
    }
}
impl Spaces {
    pub(crate) async fn authorize_owner_management(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        id: SpaceId,
    ) -> Result<OwnerManagementAccess, SpaceError> {
        let mut space = bounded(self.store.load(id)).await?;
        if space.guild != self.settings.guild
            || space.member != member
            || !(self.settings.permits_onboarding(channel)
                || (space.state != SpaceState::Retired && space.channel == Some(channel)))
        {
            return Err(SpaceError::Denied);
        }
        verify_legacy(&space, member, &self.legacy)?;
        space = bounded(self.store.expire_prepared(space.id)).await?;
        let current = bounded(self.directory.member(space.guild, member)).await?;
        if !current.human || current.pending {
            return Err(SpaceError::Denied);
        }
        let incarnation = current.joined_at.ok_or(SpaceError::Denied)?;
        if space.state != SpaceState::Retired && Some(incarnation) != space.owner_joined_at {
            return Err(SpaceError::Denied);
        }
        Ok(OwnerManagementAccess {
            space: id,
            revision: space.revision,
            generation: space.generation,
            incarnation,
        })
    }
    pub(crate) async fn authorize_finance(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        bot: MemberRef,
    ) -> Result<FinancialAccess, SpaceError> {
        if !self.connected.load(std::sync::atomic::Ordering::Acquire) {
            return Err(SpaceError::Pending);
        }
        let mut space = bounded(self.store.find(self.settings.guild, member))
            .await?
            .ok_or(SpaceError::Denied)?;
        verify_legacy(&space, member, &self.legacy)?;
        if space.channel != Some(channel)
            || !matches!(space.state, SpaceState::Ready | SpaceState::Verifying)
        {
            return Err(SpaceError::Denied);
        }
        let current = bounded(self.directory.member(space.guild, member)).await?;
        if verify_owner(&space, &current).is_err() {
            self.invalidate(super::LifecycleEvent::MemberRemoved(member))
                .await?;
            return Err(SpaceError::Denied);
        }
        if let Some(id) = space.current_operation.clone() {
            let op = bounded(self.store.operation(id)).await?;
            if op.kind == OperationKind::Create || op.phase != OperationPhase::Prepared {
                return Err(SpaceError::Pending);
            }
        }
        let snapshot = self.observe_bound_channel(&space, bot).await?;
        let viewers = bounded(self.store.viewers(space.clone())).await?;
        if self.validate_viewers(&space, bot, &viewers).await.is_err()
            || verify_privacy(&space, bot, &viewers, &snapshot, chrono::Utc::now()).is_err()
        {
            let _ = bounded(self.store.transition(SpaceTransition {
                space: space.id,
                expected_revision: space.revision,
                generation: space.generation,
                state: SpaceState::Suspended,
                source: "privacy:denied".into(),
                actor: member,
                action: AccessAction::Invalidated,
            }))
            .await;
            return Err(SpaceError::Permission);
        }
        if space.state == SpaceState::Verifying {
            space = bounded(self.store.transition(SpaceTransition {
                space: space.id,
                expected_revision: space.revision,
                generation: space.generation,
                state: SpaceState::Ready,
                source: "privacy:verified".into(),
                actor: member,
                action: AccessAction::Verified,
            }))
            .await?;
        }
        let account =
            DiscordAccountId::parse(&member.get().to_string()).map_err(|_| SpaceError::Denied)?;
        Ok(FinancialAccess {
            binding: OwnerBinding::new(
                account,
                space.journal_owner,
                crate::features::identity::BindingState::Enabled,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::identity::BindingState;
    #[test]
    fn enabled_exact_legacy_binding_is_reused_and_disabled_actor_is_reserved() {
        let actor = DiscordAccountId::parse("11").unwrap();
        let owner = OwnerRef::new("synthetic-owner".into()).unwrap();
        for state in [BindingState::Enabled, BindingState::Disabled] {
            let binding = OwnerBinding::new(actor, owner.clone(), state);
            assert!(
                legacy_owner(MemberRef::parse("22").unwrap(), &binding)
                    .unwrap()
                    .is_none()
            );
            let result = legacy_owner(MemberRef::parse("11").unwrap(), &binding);
            if state == BindingState::Enabled {
                assert_eq!(result.unwrap(), Some(&owner));
            } else {
                assert_eq!(result, Err(SpaceError::Denied));
            }
        }
    }
}
