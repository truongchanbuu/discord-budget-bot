use super::*;
impl ChannelDirectory for MemoryDirectory {
    fn create(&self, input: ChannelCreation) -> SpaceFuture<'_, ChannelRef> {
        Box::pin(async move { directory::create(self, input) })
    }
    fn change_viewer(
        &self,
        channel: ChannelRef,
        member: MemberRef,
        access: DesiredAccess,
    ) -> SpaceFuture<'_, ()> {
        Box::pin(async move { directory::change_viewer(self, channel, member, access) })
    }
    fn candidates(
        &self,
        guild: ServerRef,
        category: ChannelRef,
        generation: String,
    ) -> SpaceFuture<'_, Vec<ChannelRef>> {
        Box::pin(async move {
            Ok(self
                .channels
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .filter(|c| {
                    c.guild == guild
                        && c.parent == Some(category)
                        && c.marker.as_deref() == Some(format!("fgspace:{generation}").as_str())
                })
                .map(|c| c.channel)
                .collect())
        })
    }
    fn setup(
        &self,
        _guild: ServerRef,
        _category: ChannelRef,
        _onboarding: Vec<ChannelRef>,
        _bot: MemberRef,
    ) -> SpaceFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }

    fn member(&self, guild: ServerRef, member: MemberRef) -> SpaceFuture<'_, MemberSnapshot> {
        Box::pin(async move {
            self.members
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .find(|(g, m)| *g == guild && m.member == member)
                .map(|(_, m)| m.clone())
                .ok_or(SpaceError::Denied)
        })
    }
    fn inspect(&self, channel: ChannelRef, _bot: MemberRef) -> SpaceFuture<'_, ChannelSnapshot> {
        Box::pin(async move {
            if self
                .missing
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .contains(&channel)
            {
                return Err(SpaceError::ChannelMissing);
            }
            if self.fail_inspect.load(std::sync::atomic::Ordering::Relaxed) {
                return Err(SpaceError::Permission);
            }
            self.channels
                .lock()
                .map_err(|_| SpaceError::Storage)?
                .iter()
                .find(|c| c.channel == channel)
                .cloned()
                .map(|mut snapshot| {
                    snapshot.observed_at = chrono::Utc::now();
                    snapshot
                })
                .ok_or(SpaceError::Permission)
        })
    }
}
