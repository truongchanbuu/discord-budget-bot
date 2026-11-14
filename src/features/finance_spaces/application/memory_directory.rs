use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT_CHANNEL: AtomicU64 = AtomicU64::new(1000);

pub(super) fn create(
    directory: &MemoryDirectory,
    input: ChannelCreation,
) -> Result<ChannelRef, SpaceError> {
    directory.creates.fetch_add(1, Ordering::Relaxed);
    let number = input
        .guild
        .get()
        .checked_mul(1_000_000)
        .and_then(|base| base.checked_add(NEXT_CHANNEL.fetch_add(1, Ordering::Relaxed)))
        .ok_or(SpaceError::Invalid)?;
    if directory.fail_create.load(Ordering::Relaxed) {
        return Err(SpaceError::Uncertain);
    }
    let channel = ChannelRef::parse(&number.to_string())?;
    directory
        .channels
        .lock()
        .map_err(|_| SpaceError::Storage)?
        .push(ChannelSnapshot {
            channel,
            guild: input.guild,
            parent: Some(input.category),
            text: true,
            marker: Some(format!("fgspace:{}", input.generation)),
            bot_capable: true,
            overwrites: vec![
                Overwrite {
                    subject: Subject::Everyone,
                    profile: AccessProfile::Hidden,
                },
                Overwrite {
                    subject: Subject::Member(input.owner),
                    profile: AccessProfile::Writable,
                },
                Overwrite {
                    subject: Subject::Member(input.bot),
                    profile: AccessProfile::Writable,
                },
            ],
            observed_at: chrono::Utc::now(),
        });
    Ok(channel)
}
pub(super) fn change_viewer(
    directory: &MemoryDirectory,
    channel: ChannelRef,
    member: MemberRef,
    access: DesiredAccess,
) -> Result<(), SpaceError> {
    directory.changes.fetch_add(1, Ordering::Relaxed);
    if directory.fail_change.load(Ordering::Relaxed) {
        return Err(SpaceError::Uncertain);
    }
    let mut channels = directory.channels.lock().map_err(|_| SpaceError::Storage)?;
    let channel = channels
        .iter_mut()
        .find(|c| c.channel == channel)
        .ok_or(SpaceError::Permission)?;
    channel
        .overwrites
        .retain(|o| o.subject != Subject::Member(member));
    if access == DesiredAccess::Viewer {
        channel.overwrites.push(Overwrite {
            subject: Subject::Member(member),
            profile: AccessProfile::Viewer,
        });
    }
    Ok(())
}
