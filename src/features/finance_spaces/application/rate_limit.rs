use super::*;
use std::{collections::BTreeMap, time::Instant};
#[derive(Default)]
pub(super) struct CommandLimiter {
    actors: BTreeMap<MemberRef, (Instant, u8)>,
}
impl CommandLimiter {
    fn admit(&mut self, member: MemberRef, now: Instant) -> Result<(), SpaceError> {
        self.actors.retain(|_, (start, _)| {
            now.saturating_duration_since(*start) < Duration::from_secs(60)
        });
        if !self.actors.contains_key(&member) && self.actors.len() >= 1000 {
            return Err(SpaceError::RateLimited);
        }
        let (_, count) = self.actors.entry(member).or_insert((now, 0));
        if *count >= 10 {
            return Err(SpaceError::RateLimited);
        }
        *count += 1;
        Ok(())
    }
}
impl Spaces {
    pub(crate) fn admit_command(&self, member: MemberRef) -> Result<(), SpaceError> {
        self.limiter
            .lock()
            .map_err(|_| SpaceError::Storage)?
            .admit(member, Instant::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tenth_command_is_allowed_and_the_minute_boundary_resets() {
        let mut limit = CommandLimiter::default();
        let now = Instant::now();
        let member = MemberRef::parse("11").unwrap();
        for _ in 0..10 {
            assert!(limit.admit(member, now).is_ok());
        }
        assert_eq!(
            limit.admit(member, now + Duration::from_secs(59)),
            Err(SpaceError::RateLimited)
        );
        assert!(limit.admit(member, now + Duration::from_secs(60)).is_ok());
    }
    #[test]
    fn actor_state_is_bounded_without_evicting_active_limits() {
        let mut limit = CommandLimiter::default();
        let now = Instant::now();
        for id in 1..=1000 {
            limit
                .admit(MemberRef::parse(&id.to_string()).unwrap(), now)
                .unwrap();
        }
        assert_eq!(
            limit.admit(MemberRef::parse("1001").unwrap(), now),
            Err(SpaceError::RateLimited)
        );
        assert!(
            limit
                .admit(
                    MemberRef::parse("1001").unwrap(),
                    now + Duration::from_secs(60)
                )
                .is_ok()
        );
    }
}
