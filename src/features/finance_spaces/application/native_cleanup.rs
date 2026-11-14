use super::*;
impl Spaces {
    pub(super) async fn cleanup_invalidated(
        &self,
        mut space: FinanceSpace,
        bot: MemberRef,
    ) -> Result<FinanceSpace, SpaceError> {
        let grants = bounded(self.store.viewers(space.clone())).await?;
        for grant in grants.iter().filter(|v| {
            v.desired == ViewerDesired::Viewer && v.confirmed == ViewerConfirmed::Viewer
        }) {
            let current = bounded(self.directory.member(space.guild, grant.member)).await?;
            if current.joined_at.is_none() {
                return Err(SpaceError::Permission);
            }
            if current.joined_at != Some(grant.joined_at)
                || eligible_viewer(space.member, bot, &current).is_err()
            {
                self.invalidate(LifecycleEvent::MemberChanged(grant.member))
                    .await?;
            }
        }
        space = bounded(self.store.load(space.id)).await?;
        let grants = bounded(self.store.viewers(space.clone())).await?;
        for grant in grants.into_iter().filter(|v| {
            v.desired == ViewerDesired::Revoked && v.confirmed != ViewerConfirmed::Absent
        }) {
            if space.current_operation.is_some() {
                return Err(SpaceError::Pending);
            }
            let operation = bounded(self.store.prepare_share(SharingRequest {
                authority: SharingAuthority::InvalidatedGrant,
                space: space.id,
                revision: space.revision,
                generation: space.generation.clone(),
                actor: bot,
                target: grant.member,
                joined_at: grant.joined_at,
                kind: OperationKind::RevokeViewer,
                source: format!("lifecycle:revoke:{}:{}", grant.member.get(), space.revision),
            }))
            .await?;
            space = bounded(self.store.load(space.id)).await?;
            space = self.execute_cleanup(space, operation, bot).await?;
        }
        Ok(space)
    }
    pub(super) async fn execute_cleanup(
        &self,
        space: FinanceSpace,
        op: SpaceOperation,
        bot: MemberRef,
    ) -> Result<FinanceSpace, SpaceError> {
        if op.actor != bot
            || op.kind != OperationKind::RevokeViewer
            || !op.source.starts_with("lifecycle:revoke:")
            || op.phase != OperationPhase::Prepared
        {
            return Err(SpaceError::Pending);
        }
        let target = op.target.ok_or(SpaceError::Invalid)?;
        let channel = space.channel.ok_or(SpaceError::Permission)?;
        let active = bounded(
            self.store
                .advance(space, op.clone(), OperationStep::Dispatch),
        )
        .await?;
        let op = bounded(self.store.operation(op.id)).await?;
        let removed = tokio::time::timeout(
            Duration::from_secs(10),
            self.directory
                .change_viewer(channel, target, DesiredAccess::Absent),
        )
        .await;
        let verified = async {
            if !matches!(removed, Ok(Ok(()))) {
                return Err(SpaceError::Uncertain);
            }
            let snapshot = bounded(self.directory.inspect(channel, bot)).await?;
            if snapshot.guild != active.guild
                || snapshot.parent != Some(active.category)
                || !snapshot.text
                || snapshot.marker.as_deref()
                    != Some(format!("fgspace:{}", active.generation).as_str())
                || snapshot
                    .overwrites
                    .iter()
                    .any(|entry| entry.subject == Subject::Member(target))
            {
                return Err(SpaceError::Permission);
            }
            Ok(())
        }
        .await;
        if verified.is_err() {
            let _ = bounded(self.store.advance(active, op, OperationStep::Uncertain)).await;
            return Err(SpaceError::Uncertain);
        }
        let candidate = bounded(self.store.advance(
            active,
            op.clone(),
            OperationStep::Candidate(channel),
        ))
        .await?;
        let op = bounded(self.store.operation(op.id)).await?;
        bounded(
            self.store
                .advance(candidate, op, OperationStep::ConfirmRemoval),
        )
        .await
    }
}
