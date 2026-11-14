use super::*;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub(crate) struct MemberSnapshot {
    pub(crate) member: MemberRef,
    pub(crate) joined_at: Option<DateTime<Utc>>,
    pub(crate) human: bool,
    pub(crate) pending: bool,
    pub(crate) elevated: bool,
}
impl std::fmt::Debug for MemberSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MemberSnapshot([redacted])")
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Subject {
    Everyone,
    Member(MemberRef),
    Role(u64),
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessProfile {
    Hidden,
    Writable,
    Viewer,
    Other,
}
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Overwrite {
    pub(crate) subject: Subject,
    pub(crate) profile: AccessProfile,
}
#[derive(Clone)]
pub(crate) struct ChannelSnapshot {
    pub(crate) channel: ChannelRef,
    pub(crate) guild: ServerRef,
    pub(crate) parent: Option<ChannelRef>,
    pub(crate) text: bool,
    pub(crate) marker: Option<String>,
    pub(crate) bot_capable: bool,
    pub(crate) overwrites: Vec<Overwrite>,
    pub(crate) observed_at: DateTime<Utc>,
}
impl std::fmt::Debug for ChannelSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ChannelSnapshot([redacted])")
    }
}
pub(crate) fn verify_owner(
    space: &FinanceSpace,
    member: &MemberSnapshot,
) -> Result<(), SpaceError> {
    if member.member != space.member
        || !member.human
        || member.pending
        || member.joined_at.is_none()
        || member.joined_at != space.owner_joined_at
    {
        return Err(SpaceError::Denied);
    }
    Ok(())
}
pub(crate) fn eligible_viewer(
    owner: MemberRef,
    bot: MemberRef,
    target: &MemberSnapshot,
) -> Result<(), SpaceError> {
    if target.member == owner
        || target.member == bot
        || !target.human
        || target.pending
        || target.elevated
        || target.joined_at.is_none()
    {
        return Err(SpaceError::Denied);
    }
    Ok(())
}
pub(crate) fn verify_privacy(
    space: &FinanceSpace,
    bot: MemberRef,
    viewers: &[ViewerGrant],
    snapshot: &ChannelSnapshot,
    now: DateTime<Utc>,
) -> Result<(), SpaceError> {
    let age = now.signed_duration_since(snapshot.observed_at);
    if space.channel != Some(snapshot.channel)
        || snapshot.guild != space.guild
        || snapshot.parent != Some(space.category)
        || !snapshot.text
        || !snapshot.bot_capable
        || age < chrono::Duration::zero()
        || age > chrono::Duration::seconds(5)
        || snapshot.marker.as_deref() != Some(format!("fgspace:{}", space.generation).as_str())
    {
        return Err(SpaceError::Permission);
    }
    let mut expected = vec![
        Overwrite {
            subject: Subject::Everyone,
            profile: AccessProfile::Hidden,
        },
        Overwrite {
            subject: Subject::Member(space.member),
            profile: AccessProfile::Writable,
        },
        Overwrite {
            subject: Subject::Member(bot),
            profile: AccessProfile::Writable,
        },
    ];
    for viewer in viewers {
        if viewer.generation != space.generation
            || viewer.desired != ViewerDesired::Viewer
            || viewer.confirmed != ViewerConfirmed::Viewer
        {
            continue;
        }
        if viewer.space != space.id {
            return Err(SpaceError::Permission);
        }
        viewer.validate(space.member, bot)?;
        let entry = Overwrite {
            subject: Subject::Member(viewer.member),
            profile: AccessProfile::Viewer,
        };
        if expected.contains(&entry) || expected.len() >= 11 {
            return Err(SpaceError::Permission);
        }
        expected.push(entry);
    }
    if snapshot.overwrites.len() != expected.len()
        || expected.iter().any(|entry| {
            snapshot
                .overwrites
                .iter()
                .filter(|actual| *actual == entry)
                .count()
                != 1
        })
    {
        return Err(SpaceError::Permission);
    }
    Ok(())
}
