use super::*;
impl Spaces {
    pub(crate) async fn invalidate(&self, event: LifecycleEvent) -> Result<(), SpaceError> {
        bounded(self.store.invalidate(self.settings.guild, event)).await
    }
    pub(crate) async fn check_legacy(&self) -> Result<(), SpaceError> {
        let member = MemberRef::parse(&self.legacy.configured_account().as_u64().to_string())?;
        let owner = authorization::legacy_owner(member, &self.legacy)
            .ok()
            .flatten()
            .cloned();
        bounded(self.store.check_legacy(LegacyCheck {
            guild: self.settings.guild,
            member,
            owner,
        }))
        .await
    }
    pub(crate) async fn request_recovery(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        source: &str,
    ) -> Result<FinanceSpace, SpaceError> {
        if !self.settings.permits_onboarding(channel) {
            return Err(SpaceError::Denied);
        }
        authorization::legacy_owner(member, &self.legacy)?;
        let owner = bounded(self.directory.member(self.settings.guild, member)).await?;
        if !owner.human || owner.pending {
            return Err(SpaceError::Denied);
        }
        let mut space = bounded(self.store.find(self.settings.guild, member))
            .await?
            .ok_or(SpaceError::Denied)?;
        authorization::verify_legacy(&space, member, &self.legacy)?;
        if owner.joined_at.is_none() {
            return Err(SpaceError::Denied);
        }
        if space.owner_joined_at != owner.joined_at {
            self.invalidate(LifecycleEvent::MemberRemoved(member))
                .await?;
            space = bounded(self.store.load(space.id)).await?;
        }
        if space.state != SpaceState::Retired {
            return Err(SpaceError::Pending);
        }
        if space.current_operation.is_some() && space.owner_joined_at == owner.joined_at {
            return Ok(space);
        }
        bounded(self.store.recover(RecoveryRequest {
            space: space.id,
            expected_revision: space.revision,
            category: self.settings.category,
            member,
            joined_at: owner.joined_at.ok_or(SpaceError::Denied)?,
            source: source.into(),
        }))
        .await
    }
    pub(crate) async fn confirm_recovery(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        id: SpaceId,
        revision: i64,
        bot: MemberRef,
    ) -> Result<FinanceSpace, SpaceError> {
        if !self.settings.permits_onboarding(channel) {
            return Err(SpaceError::Denied);
        }
        let space = bounded(self.store.load(id)).await?;
        if space.guild != self.settings.guild
            || space.member != member
            || space.revision != revision
            || space.state != SpaceState::Retired
        {
            return Err(SpaceError::Denied);
        }
        authorization::verify_legacy(&space, member, &self.legacy)?;
        let owner = bounded(self.directory.member(space.guild, member)).await?;
        verify_owner(&space, &owner)?;
        self.provision(space, bot).await
    }
    pub(crate) async fn reconcile(
        &self,
        space: FinanceSpace,
        bot: MemberRef,
    ) -> Result<(), SpaceError> {
        let mut space = bounded(self.store.expire_prepared(space.id)).await?;
        if space.state == SpaceState::Retired {
            return Ok(());
        }
        let owner = bounded(self.directory.member(space.guild, space.member)).await?;
        if verify_owner(&space, &owner).is_err() {
            self.invalidate(LifecycleEvent::MemberRemoved(space.member))
                .await?;
            return Err(SpaceError::Denied);
        }
        if let Some(id) = space.current_operation.clone() {
            let op = bounded(self.store.operation(id)).await?;
            if op.kind != OperationKind::Create {
                self.observe_bound_channel(&space, bot).await?;
                self.execute_cleanup(space, op, bot).await?;
                return Ok(());
            }
            if op.phase == OperationPhase::Prepared {
                return Ok(());
            }
            let candidates = bounded(self.directory.candidates(
                space.guild,
                space.category,
                space.generation.clone(),
            ))
            .await?;
            if candidates.len() != 1 {
                return Err(SpaceError::Uncertain);
            }
            if space
                .channel
                .is_some_and(|channel| channel != candidates[0])
                || op
                    .returned_channel
                    .is_some_and(|channel| channel != candidates[0])
            {
                return Err(SpaceError::Uncertain);
            }
            let mut candidate = space.clone();
            candidate.channel = Some(candidates[0]);
            let snapshot = bounded(self.directory.inspect(candidates[0], bot)).await?;
            verify_privacy(&candidate, bot, &[], &snapshot, chrono::Utc::now())?;
            let verified = if op.phase == OperationPhase::Verifying {
                space
            } else {
                bounded(self.store.advance(
                    space,
                    op.clone(),
                    OperationStep::Candidate(candidates[0]),
                ))
                .await?
            };
            let fresh = bounded(self.store.operation(op.id)).await?;
            bounded(self.store.advance(verified, fresh, OperationStep::Confirm)).await?;
        } else if matches!(
            space.state,
            SpaceState::Ready | SpaceState::Verifying | SpaceState::Suspended
        ) {
            space = self.cleanup_invalidated(space, bot).await?;
            let viewers = bounded(self.store.viewers(space.clone())).await?;
            self.validate_viewers(&space, bot, &viewers).await?;
            let snapshot = self.observe_bound_channel(&space, bot).await?;
            verify_privacy(&space, bot, &viewers, &snapshot, chrono::Utc::now())?;
            if space.state == SpaceState::Ready {
                return Ok(());
            }
            bounded(self.store.transition(SpaceTransition {
                space: space.id,
                expected_revision: space.revision,
                generation: space.generation,
                state: SpaceState::Ready,
                source: "lifecycle:verified".into(),
                actor: space.member,
                action: AccessAction::Verified,
            }))
            .await?;
        }
        Ok(())
    }
    pub(crate) async fn reconcile_batch(
        &self,
        after: i64,
        bot: MemberRef,
    ) -> Result<Option<i64>, SpaceError> {
        let batch = bounded(self.store.tracked(self.settings.guild, after)).await?;
        let next = batch.last().map(|s| s.id.get());
        for space in batch {
            let _ = tokio::time::timeout(Duration::from_secs(20), self.reconcile(space, bot)).await;
        }
        Ok(next)
    }
}

impl Spaces {
    pub(super) async fn observe_bound_channel(
        &self,
        space: &FinanceSpace,
        bot: MemberRef,
    ) -> Result<ChannelSnapshot, SpaceError> {
        let channel = space.channel.ok_or(SpaceError::Permission)?;
        match bounded(self.directory.inspect(channel, bot)).await {
            Err(SpaceError::ChannelMissing) => {
                self.invalidate(LifecycleEvent::ChannelDeleted(channel))
                    .await?;
                Err(SpaceError::ChannelMissing)
            }
            result => result,
        }
    }
}
