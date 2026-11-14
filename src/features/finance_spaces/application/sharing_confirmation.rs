use super::*;
impl Spaces {
    pub(crate) async fn confirm_viewer(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        id: String,
        revision: i64,
        bot: MemberRef,
    ) -> Result<(), SpaceError> {
        let op = bounded(self.store.operation(id.clone())).await?;
        if op.kind == OperationKind::Create || op.actor != member {
            return Err(SpaceError::Denied);
        }
        let access = self
            .authorize_owner_management(member, channel, op.space)
            .await?;
        let space = bounded(self.store.load(op.space)).await?;
        access.check_current(&space)?;
        if op.generation != space.generation {
            return Err(SpaceError::Denied);
        }
        if op.phase == OperationPhase::Confirmed {
            self.authorize_finance(member, channel, bot).await?;
            return Ok(());
        }
        if op.phase != OperationPhase::Prepared
            || space.revision != revision
            || op.expected_revision != revision
            || op.prepared_expired(chrono::Utc::now())
        {
            return Err(SpaceError::Conflict);
        }
        if op.kind == OperationKind::GrantViewer {
            self.authorize_finance(member, channel, bot).await?;
            let target = bounded(
                self.directory
                    .member(space.guild, op.target.ok_or(SpaceError::Invalid)?),
            )
            .await?;
            eligible_viewer(member, bot, &target)?;
            if target.joined_at != op.target_joined_at {
                return Err(SpaceError::Denied);
            }
        }
        let mut active = bounded(
            self.store
                .advance(space, op.clone(), OperationStep::Dispatch),
        )
        .await?;
        let op = bounded(self.store.operation(id)).await?;
        let change = tokio::time::timeout(
            Duration::from_secs(10),
            self.directory.change_viewer(
                channel,
                op.target.ok_or(SpaceError::Invalid)?,
                op.desired_access.ok_or(SpaceError::Invalid)?,
            ),
        )
        .await;
        if !matches!(change, Ok(Ok(()))) {
            let _ = bounded(self.store.advance(active, op, OperationStep::Uncertain)).await;
            return Err(SpaceError::Uncertain);
        }
        let mut viewers = bounded(self.store.viewers(active.clone())).await?;
        let target = viewers
            .iter_mut()
            .find(|v| Some(v.member) == op.target)
            .ok_or(SpaceError::Conflict)?;
        target.confirmed = if op.kind == OperationKind::GrantViewer {
            ViewerConfirmed::Viewer
        } else {
            ViewerConfirmed::Absent
        };
        let verification = async {
            self.validate_viewers(&active, bot, &viewers).await?;
            let snapshot = bounded(self.directory.inspect(channel, bot)).await?;
            verify_privacy(&active, bot, &viewers, &snapshot, chrono::Utc::now())?;
            let owner = bounded(self.directory.member(active.guild, member)).await?;
            verify_owner(&active, &owner)
        }
        .await;
        if verification.is_err() {
            let _ = bounded(self.store.advance(active, op, OperationStep::Uncertain)).await;
            return Err(SpaceError::Uncertain);
        }
        active = bounded(
            self.store
                .advance(active, op.clone(), OperationStep::Candidate(channel)),
        )
        .await?;
        let op = bounded(self.store.operation(op.id)).await?;
        bounded(self.store.advance(active, op, OperationStep::Confirm)).await?;
        Ok(())
    }
}
