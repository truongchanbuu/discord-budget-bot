use super::*;
impl Spaces {
    pub(crate) async fn list_grants(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        id: SpaceId,
    ) -> Result<Vec<ViewerGrant>, SpaceError> {
        self.authorize_owner_management(member, channel, id).await?;
        let space = bounded(self.store.expire_prepared(id)).await?;
        bounded(self.store.viewers(space)).await
    }
    pub(crate) async fn prepare_viewer(
        &self,
        input: ViewerRequest,
        bot: MemberRef,
    ) -> Result<SpaceOperation, SpaceError> {
        let access = self
            .authorize_owner_management(input.member, input.channel, input.space)
            .await?;
        let space = bounded(self.store.load(input.space)).await?;
        access.check_current(&space)?;
        let joined_at = match input.kind {
            OperationKind::GrantViewer => {
                self.authorize_finance(input.member, input.channel, bot)
                    .await?;
                let target = bounded(self.directory.member(space.guild, input.target)).await?;
                eligible_viewer(space.member, bot, &target)?;
                target.joined_at.ok_or(SpaceError::Denied)?
            }
            OperationKind::RevokeViewer => {
                bounded(self.store.viewers(space.clone()))
                    .await?
                    .into_iter()
                    .find(|v| v.member == input.target)
                    .ok_or(SpaceError::Denied)?
                    .joined_at
            }
            OperationKind::Create => return Err(SpaceError::Invalid),
        };
        bounded(self.store.prepare_share(SharingRequest {
            authority: SharingAuthority::Owner,
            space: space.id,
            revision: space.revision,
            generation: space.generation,
            actor: input.member,
            target: input.target,
            joined_at,
            kind: input.kind,
            source: input.source,
        }))
        .await
    }
    pub(crate) async fn cancel_operation(
        &self,
        member: MemberRef,
        channel: ChannelRef,
        id: String,
        revision: i64,
    ) -> Result<(), SpaceError> {
        let op = bounded(self.store.operation(id.clone())).await?;
        let access = self
            .authorize_owner_management(member, channel, op.space)
            .await?;
        let space = bounded(self.store.load(op.space)).await?;
        access.check_current(&space)?;
        if space.revision != revision || op.generation != space.generation || op.actor != member {
            return Err(SpaceError::Denied);
        }
        bounded(self.store.cancel_prepared(space, id)).await?;
        Ok(())
    }
    pub(super) async fn validate_viewers(
        &self,
        space: &FinanceSpace,
        bot: MemberRef,
        viewers: &[ViewerGrant],
    ) -> Result<(), SpaceError> {
        bounded(async {
            for viewer in viewers.iter().filter(|v| {
                v.generation == space.generation
                    && v.desired == ViewerDesired::Viewer
                    && v.confirmed == ViewerConfirmed::Viewer
            }) {
                let member = self.directory.member(space.guild, viewer.member).await?;
                eligible_viewer(space.member, bot, &member)?;
                if member.joined_at != Some(viewer.joined_at) {
                    return Err(SpaceError::Permission);
                }
            }
            Ok(())
        })
        .await
    }
}

impl Spaces {
    pub(crate) async fn management_status(
        &self,
        member: MemberRef,
        channel: ChannelRef,
    ) -> Result<OwnerManagementAccess, SpaceError> {
        let space = bounded(self.store.find(self.settings.guild, member))
            .await?
            .ok_or(SpaceError::Denied)?;
        self.authorize_owner_management(member, channel, space.id)
            .await
    }
}
