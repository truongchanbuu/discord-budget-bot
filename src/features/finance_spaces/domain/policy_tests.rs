use super::model_tests::space;
use super::*;
use chrono::Utc;

fn member(id: &str) -> MemberSnapshot {
    MemberSnapshot {
        member: MemberRef::parse(id).unwrap(),
        joined_at: Some(Utc::now()),
        human: true,
        pending: false,
        elevated: false,
    }
}
fn snapshot(s: &FinanceSpace) -> ChannelSnapshot {
    ChannelSnapshot {
        channel: s.channel.unwrap(),
        guild: s.guild,
        parent: Some(s.category),
        text: true,
        marker: Some(format!("fgspace:{}", s.generation)),
        bot_capable: true,
        observed_at: Utc::now(),
        overwrites: vec![
            Overwrite {
                subject: Subject::Everyone,
                profile: AccessProfile::Hidden,
            },
            Overwrite {
                subject: Subject::Member(s.member),
                profile: AccessProfile::Writable,
            },
            Overwrite {
                subject: Subject::Member(MemberRef::parse("99").unwrap()),
                profile: AccessProfile::Writable,
            },
        ],
    }
}
#[test]
fn exact_privacy_rejects_drift_missing_and_unknown_facts() {
    let s = space();
    let bot = MemberRef::parse("99").unwrap();
    let good = snapshot(&s);
    assert!(verify_privacy(&s, bot, &[], &good, Utc::now()).is_ok());
    let mut bad = good.clone();
    bad.overwrites.push(Overwrite {
        subject: Subject::Role(123),
        profile: AccessProfile::Writable,
    });
    assert_eq!(
        verify_privacy(&s, bot, &[], &bad, Utc::now()),
        Err(SpaceError::Permission)
    );
    bad = good.clone();
    bad.parent = None;
    assert!(verify_privacy(&s, bot, &[], &bad, Utc::now()).is_err());
    bad = good.clone();
    bad.bot_capable = false;
    assert!(verify_privacy(&s, bot, &[], &bad, Utc::now()).is_err());
    bad = good.clone();
    bad.marker = None;
    assert!(verify_privacy(&s, bot, &[], &bad, Utc::now()).is_err());
    assert!(
        verify_privacy(
            &s,
            bot,
            &[],
            &good,
            Utc::now() + chrono::Duration::seconds(6)
        )
        .is_err()
    );
}
#[test]
fn owner_checks_use_current_incarnation_and_explicit_identity() {
    let s = space();
    let mut owner = member("11");
    owner.joined_at = s.owner_joined_at;
    assert!(verify_owner(&s, &owner).is_ok());
    assert!(verify_owner(&s, &member("22")).is_err());
    owner.joined_at = None;
    assert!(verify_owner(&s, &owner).is_err());
    owner.joined_at = s.owner_joined_at;
    owner.pending = true;
    assert!(verify_owner(&s, &owner).is_err());
    owner.pending = false;
    owner.human = false;
    assert!(verify_owner(&s, &owner).is_err());
}
#[test]
fn viewers_must_be_current_humans_without_elevated_access() {
    let s = space();
    let bot = MemberRef::parse("99").unwrap();
    let mut target = member("22");
    assert!(eligible_viewer(s.member, bot, &target).is_ok());
    target.elevated = true;
    assert!(eligible_viewer(s.member, bot, &target).is_err());
    target.elevated = false;
    target.pending = true;
    assert!(eligible_viewer(s.member, bot, &target).is_err());
    assert!(eligible_viewer(s.member, bot, &member("11")).is_err());
    assert!(eligible_viewer(s.member, bot, &member("99")).is_err());
}
