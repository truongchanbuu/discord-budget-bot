use super::support::{account, actor, binding, owner};
use fundguard::features::identity::{
    AccessError, BindingState, DiscordAccountId, authorize_scope, resolve_owner,
};
use std::error::Error;

#[test]
fn diagnostic_formats_redact_all_identity_payloads() {
    let account_marker = "420123456789";
    let owner_marker = "owner-sensitive-marker-A";
    let account = account(account_marker);
    let actor = actor(account_marker);
    let binding = binding(account_marker, owner_marker, BindingState::Enabled);
    let scope = owner(owner_marker);
    let resolved = resolve_owner(Some(&actor), Some(&binding));
    let authorized = authorize_scope(Some(&actor), Some(&binding), Some(&scope));
    let nested = (&account, &actor, &binding, &resolved, &authorized);
    for output in [
        format!("{account:?}"),
        format!("{account:#?}"),
        format!("{actor:?}"),
        format!("{actor:#?}"),
        format!("{binding:?}"),
        format!("{binding:#?}"),
        format!("{resolved:?}"),
        format!("{authorized:#?}"),
        format!("{nested:?}"),
        format!("{nested:#?}"),
    ] {
        assert!(!output.contains(account_marker));
        assert!(!output.contains(owner_marker));
    }
}

#[test]
fn every_error_is_categorical_without_retained_input_or_sources() {
    let account_marker = "420123456789";
    let owner_marker = "owner-sensitive-marker-B";
    let unknown = actor("430123456789");
    let binding = binding(account_marker, owner_marker, BindingState::Enabled);
    let malformed = format!("+{account_marker}");
    let errors = [
        DiscordAccountId::parse(&malformed).unwrap_err(),
        resolve_owner(Some(&unknown), Some(&binding)).unwrap_err(),
        authorize_scope(None, None, Some(&owner(owner_marker))).unwrap_err(),
    ];
    assert_eq!(
        errors,
        [
            AccessError::InvalidAccountId,
            AccessError::Denied,
            AccessError::Denied
        ]
    );
    for error in errors {
        assert!(error.source().is_none());
        for output in [
            error.to_string(),
            format!("{error:?}"),
            format!("{error:#?}"),
        ] {
            assert!(!output.contains(account_marker));
            assert!(!output.contains(owner_marker));
            assert!(!output.contains(&malformed));
        }
    }
}

#[test]
fn policy_is_independent_of_interaction_labels_and_has_no_remembered_grant() {
    let known = actor("42");
    let unknown = actor("43");
    let binding = binding("42", "owner-A", BindingState::Enabled);
    let scope = owner("owner-A");
    let resolved = resolve_owner(Some(&known), Some(&binding)).unwrap();
    for _interaction in ["message", "command", "component"] {
        assert!(std::ptr::eq(
            authorize_scope(Some(&known), Some(&binding), Some(&scope)).unwrap(),
            resolved
        ));
        assert_eq!(
            authorize_scope(Some(&unknown), Some(&binding), Some(&scope)),
            Err(AccessError::Denied)
        );
        assert_eq!(
            authorize_scope(Some(&known), None, Some(&scope)),
            Err(AccessError::Denied)
        );
    }
}
