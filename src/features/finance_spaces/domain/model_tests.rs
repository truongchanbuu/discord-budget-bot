use super::*;

#[test]
fn provider_references_are_canonical_and_redacted() {
    for invalid in [
        "",
        "0",
        "01",
        "+1",
        " 1",
        "1 ",
        "18446744073709551616",
        "secret-marker",
    ] {
        assert!(ServerRef::parse(invalid).is_err());
        assert!(ChannelRef::parse(invalid).is_err());
        assert!(MemberRef::parse(invalid).is_err());
    }
    let server = ServerRef::parse("18446744073709551615").unwrap();
    assert_eq!(server.get(), u64::MAX);
    assert!(!format!("{server:?}").contains("18446744073709551615"));
    assert!(SpaceId::new(0).is_err());
    assert!(SpaceId::new(-1).is_err());
    assert_eq!(SpaceId::new(1).unwrap().get(), 1);
}

#[test]
fn readiness_requires_binding_and_incarnation() {
    assert!(SpaceState::parse("unknown").is_err());
    assert_eq!(SpaceState::parse("ready").unwrap(), SpaceState::Ready);
    for state in [
        SpaceState::Prepared,
        SpaceState::Creating,
        SpaceState::Uncertain,
        SpaceState::Verifying,
        SpaceState::AccessPending,
        SpaceState::Suspended,
        SpaceState::Retired,
    ] {
        assert!(!state.permits_finance());
    }
    assert!(SpaceState::Ready.permits_finance());
    assert_eq!(SpaceError::Denied.to_string(), "space access denied");
}

pub(super) fn space() -> FinanceSpace {
    FinanceSpace {
        id: SpaceId::new(1).unwrap(),
        guild: ServerRef::parse("10").unwrap(),
        member: MemberRef::parse("11").unwrap(),
        journal_owner: crate::features::financial_core::OwnerRef::new("secret-marker".into())
            .unwrap(),
        legacy: false,
        owner_joined_at: Some(chrono::Utc::now()),
        generation: "12345678-1234-1234-1234-123456789abc".into(),
        channel: Some(ChannelRef::parse("12").unwrap()),
        category: ChannelRef::parse("13").unwrap(),
        state: SpaceState::Ready,
        revision: 0,
        current_operation: None,
    }
}

#[test]
fn aggregate_checks_missing_bindings_and_revision_overflow() {
    let mut value = space();
    assert!(value.is_bound_ready());
    assert!(!format!("{value:?}").contains("secret-marker"));
    value.owner_joined_at = None;
    assert!(!value.is_bound_ready());
    value.owner_joined_at = Some(chrono::Utc::now());
    value.channel = None;
    assert!(!value.is_bound_ready());
    value.revision = i64::MAX;
    assert_eq!(value.next_revision(), Err(SpaceError::Conflict));
}

#[test]
fn operation_transition_and_source_validation() {
    assert!(OperationPhase::Prepared.can_transition(OperationPhase::Dispatched));
    assert!(OperationPhase::Dispatched.can_transition(OperationPhase::Uncertain));
    assert!(!OperationPhase::Uncertain.can_transition(OperationPhase::Prepared));
    assert!(!OperationPhase::Confirmed.can_transition(OperationPhase::Dispatched));
    assert!(OperationPhase::parse("made-up").is_err());
    assert!(!valid_source(""));
    assert!(!valid_source(&"a".repeat(129)));
    assert!(!valid_source("message\nsecret-marker"));
    assert!(valid_source("discord:guild:123"));
}

#[test]
fn viewer_validation_and_audit_redaction() {
    let owner = MemberRef::parse("11").unwrap();
    let bot = MemberRef::parse("99").unwrap();
    let mut viewer = ViewerGrant {
        space: SpaceId::new(1).unwrap(),
        generation: space().generation,
        member: MemberRef::parse("22").unwrap(),
        joined_at: chrono::Utc::now(),
        desired: ViewerDesired::Viewer,
        confirmed: ViewerConfirmed::Viewer,
        revision: 0,
        operation: "12345678-1234-1234-1234-123456789abc".into(),
        granted_by: owner,
        changed_at: chrono::Utc::now(),
    };
    assert!(viewer.validate(owner, bot).is_ok());
    viewer.member = owner;
    assert_eq!(viewer.validate(owner, bot), Err(SpaceError::Invalid));
    viewer.member = bot;
    assert_eq!(viewer.validate(owner, bot), Err(SpaceError::Invalid));
    assert!(!format!("{viewer:?}").contains("12345678"));
}

#[test]
fn operation_validation_and_expiry_never_release_dispatched_writes() {
    let now = chrono::Utc::now();
    let mut operation = SpaceOperation {
        id: space().generation.clone(),
        space: SpaceId::new(1).unwrap(),
        kind: OperationKind::GrantViewer,
        source: "discord:guild:100".into(),
        actor: MemberRef::parse("11").unwrap(),
        generation: space().generation,
        expected_revision: 0,
        target: Some(MemberRef::parse("22").unwrap()),
        target_joined_at: Some(now),
        phase: OperationPhase::Prepared,
        returned_channel: None,
        created_at: now,
        dispatched_at: None,
        completed_at: None,
        outcome: None,
        desired_access: Some(DesiredAccess::Viewer),
    };
    assert!(operation.validate().is_ok());
    assert!(!operation.prepared_expired(now + chrono::Duration::seconds(299)));
    assert!(operation.prepared_expired(now + chrono::Duration::seconds(300)));
    operation.phase = OperationPhase::Dispatched;
    operation.dispatched_at = Some(now);
    assert!(!operation.prepared_expired(now + chrono::Duration::hours(1)));
    operation.expected_revision = -1;
    assert_eq!(operation.validate(), Err(SpaceError::Invalid));
    operation.expected_revision = 0;
    operation.target = Some(operation.actor);
    assert_eq!(operation.validate(), Err(SpaceError::Invalid));
    operation.target = Some(MemberRef::parse("22").unwrap());
    operation.desired_access = Some(DesiredAccess::Absent);
    assert_eq!(operation.validate(), Err(SpaceError::Invalid));
    let audit = AccessEvent {
        space: operation.space,
        operation: Some(operation.id.clone()),
        source: operation.source.clone(),
        actor: operation.actor,
        action: AccessAction::Reserved,
        outcome: OperationOutcome::Pending,
        at: now,
    };
    assert!(audit.validate().is_ok());
    assert!(!format!("{audit:?} {operation:?}").contains("12345678"));
}
