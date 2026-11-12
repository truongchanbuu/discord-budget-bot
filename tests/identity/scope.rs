use super::support::{actor, binding, owner};
use fundguard::features::identity::{AccessError, BindingState, authorize_scope, resolve_owner};

#[test]
fn scope_matrix_allows_only_enabled_matching_actor_and_owner() {
    let known = actor("42");
    let unknown = actor("43");
    let enabled = binding("42", "owner-A", BindingState::Enabled);
    let disabled = binding("42", "owner-A", BindingState::Disabled);
    let matching = owner("owner-A");
    let foreign = owner("owner-B");
    for (actor_index, actor) in [None, Some(&known), Some(&unknown)].into_iter().enumerate() {
        for (binding_index, binding) in [None, Some(&enabled), Some(&disabled)]
            .into_iter()
            .enumerate()
        {
            for (scope_index, scope) in [None, Some(&matching), Some(&foreign)]
                .into_iter()
                .enumerate()
            {
                let result = authorize_scope(actor, binding, scope);
                if (actor_index, binding_index, scope_index) == (1, 1, 1) {
                    let resolved = resolve_owner(actor, binding).unwrap();
                    let authorized = result.unwrap();
                    assert!(std::ptr::eq(authorized, resolved));
                    assert!(!std::ptr::eq(authorized, &matching));
                } else {
                    assert_eq!(result, Err(AccessError::Denied));
                }
            }
        }
    }
    assert_eq!(known, actor("42"));
    assert_eq!(matching, owner("owner-A"));
    assert_eq!(
        resolve_owner(Some(&known), Some(&enabled)).unwrap(),
        &matching
    );
}

#[test]
fn exact_owner_bytes_and_current_mapping_control_every_evaluation() {
    let actor = actor("42");
    let initial = binding("42", "owner-A", BindingState::Enabled);
    let disabled = binding("42", "owner-A", BindingState::Disabled);
    let changed = binding("42", "owner-B", BindingState::Enabled);
    let owner_a = owner("owner-A");
    let owner_b = owner("owner-B");
    assert!(authorize_scope(Some(&actor), Some(&initial), Some(&owner_a)).is_ok());
    for scope in ["Owner-A", " owner-A "] {
        assert_eq!(
            authorize_scope(Some(&actor), Some(&initial), Some(&owner(scope))),
            Err(AccessError::Denied)
        );
    }
    for current in [None, Some(&disabled), Some(&changed)] {
        assert_eq!(
            authorize_scope(Some(&actor), current, Some(&owner_a)),
            Err(AccessError::Denied)
        );
    }
    assert_eq!(
        resolve_owner(Some(&actor), Some(&changed)).unwrap(),
        &owner_b
    );
    assert_eq!(
        authorize_scope(Some(&actor), Some(&changed), Some(&owner_b)).unwrap(),
        &owner_b
    );
}
