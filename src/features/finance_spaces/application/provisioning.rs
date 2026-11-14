use super::*;
impl Spaces {
    pub(crate) async fn onboard(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        bot: MemberRef,
        source: &str,
    ) -> Result<FinanceSpace, SpaceError> {
        if !self.settings.permits_onboarding(channel) || !valid_source(source) {
            return Err(SpaceError::Denied);
        }
        let owner = bounded(self.directory.member(self.settings.guild, member)).await?;
        if !owner.human || owner.pending {
            return Err(SpaceError::Denied);
        }
        let legacy_owner = authorization::legacy_owner(member, &self.legacy)?.cloned();
        let mut space = bounded(self.store.reserve(Reservation {
            guild: self.settings.guild,
            member,
            category: self.settings.category,
            joined_at: owner.joined_at.ok_or(SpaceError::Denied)?,
            legacy_owner,
            source: source.into(),
        }))
        .await?;
        verify_owner(&space, &owner)?;
        if matches!(space.state, SpaceState::Verifying | SpaceState::Suspended) {
            let _ = self.reconcile(space.clone(), bot).await;
            space = bounded(self.store.load(space.id)).await?;
        }
        if space.state == SpaceState::Ready {
            let snapshot = self.observe_bound_channel(&space, bot).await?;
            let viewers = bounded(self.store.viewers(space.clone())).await?;
            self.validate_viewers(&space, bot, &viewers).await?;
            verify_privacy(&space, bot, &viewers, &snapshot, chrono::Utc::now())?;
            return Ok(space);
        }
        if space.state != SpaceState::Prepared {
            return Err(SpaceError::Pending);
        }
        self.provision(space, bot).await
    }
    pub(super) async fn provision(
        &self,
        space: FinanceSpace,
        bot: MemberRef,
    ) -> Result<FinanceSpace, SpaceError> {
        let op = bounded(
            self.store
                .operation(space.current_operation.clone().ok_or(SpaceError::Pending)?),
        )
        .await?;
        let dispatched = bounded(
            self.store
                .advance(space, op.clone(), OperationStep::Dispatch),
        )
        .await?;
        let operation = bounded(self.store.operation(op.id)).await?;
        let create = tokio::time::timeout(
            Duration::from_secs(10),
            self.directory.create(ChannelCreation {
                guild: dispatched.guild,
                category: dispatched.category,
                owner: dispatched.member,
                bot,
                generation: dispatched.generation.clone(),
            }),
        )
        .await;
        let channel = match create {
            Ok(Ok(channel)) => channel,
            _ => {
                let _ = bounded(self.store.advance(
                    dispatched,
                    operation,
                    OperationStep::Uncertain,
                ))
                .await;
                return Err(SpaceError::Uncertain);
            }
        };
        let candidate = bounded(self.store.advance(
            dispatched,
            operation,
            OperationStep::Candidate(channel),
        ))
        .await?;
        let snapshot = bounded(self.directory.inspect(channel, bot)).await?;
        verify_privacy(&candidate, bot, &[], &snapshot, chrono::Utc::now())?;
        let owner = bounded(self.directory.member(candidate.guild, candidate.member)).await?;
        verify_owner(&candidate, &owner)?;
        let operation = bounded(
            self.store.operation(
                candidate
                    .current_operation
                    .clone()
                    .ok_or(SpaceError::Pending)?,
            ),
        )
        .await?;
        bounded(
            self.store
                .advance(candidate, operation, OperationStep::Confirm),
        )
        .await
    }
}
