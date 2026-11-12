use super::support::{actor, binding, owner};
use fundguard::features::identity::{AccessError, BindingState, resolve_owner};

#[test]
fn exact_enabled_account_resolves_the_same_bound_owner_without_mutation() {
    let actor = actor("42");
    let binding = binding("42", " Owner-A ", BindingState::Enabled);
    let first = resolve_owner(Some(&actor), Some(&binding)).unwrap();
    assert_eq!(first, &owner(" Owner-A "));
    let second = resolve_owner(Some(&actor), Some(&binding)).unwrap();
    assert!(std::ptr::eq(first, second));
    assert_eq!(actor, super::support::actor("42"));
}

#[test]
fn all_absent_disabled_and_unknown_cases_deny_uniformly() {
    let known = actor("42");
    let unknown = actor("43");
    let enabled = binding("42", "owner-A", BindingState::Enabled);
    let disabled = binding("42", "owner-A", BindingState::Disabled);
    for actor in [None, Some(&known), Some(&unknown)] {
        assert_eq!(resolve_owner(actor, None), Err(AccessError::Denied));
        assert_eq!(
            resolve_owner(actor, Some(&disabled)),
            Err(AccessError::Denied)
        );
    }
    assert_eq!(
        resolve_owner(None, Some(&enabled)),
        Err(AccessError::Denied)
    );
    assert_eq!(
        resolve_owner(Some(&unknown), Some(&enabled)),
        Err(AccessError::Denied)
    );
}
