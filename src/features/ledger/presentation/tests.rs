use super::*;
use crate::features::ledger::domain::*;
use ::serenity::json::{from_value, json, to_value};
#[test]
fn component_ids_are_bounded_and_never_authorize_an_owner() {
    for action in ["save", "edit", "cancel", "modal"] {
        let value = reference(action, 42, 3);
        assert!(value.len() <= 100);
        let parsed = parse_reference(&value).unwrap();
        assert_eq!((parsed.id, parsed.revision), (42, 3));
        assert!(!value.contains("owner"));
    }
    for value in [
        "fgfinance:save:-1:0",
        "fgfinance:save:1:-1",
        "fgfinance:unknown:1:0",
        "other:save:1:0",
    ] {
        assert!(parse_reference(value).is_err());
    }
}
#[test]
fn actual_preview_and_modal_builders_preserve_privacy() {
    let proposal = parse(
        "donate 100k synthetic-note-marker",
        date("2026-11-13").unwrap(),
    )
    .unwrap();
    let draft = Draft {
        id: 42,
        owner: crate::features::financial_core::OwnerRef::new("owner-marker".into()).unwrap(),
        channel: "100".into(),
        original_note: proposal.note.clone(),
        proposal,
        revision: 3,
        expires: chrono::Utc::now(),
        state: State::Pending,
        entry: None,
    };
    let message = to_value(views::message(
        views::preview(&draft),
        views::buttons(&draft),
    ))
    .unwrap();
    assert_eq!(message["allowed_mentions"]["parse"], json!([]));
    assert_eq!(message["flags"], 4);
    let modal = to_value(modals::builder(&draft)).unwrap();
    assert_eq!(modal["components"].as_array().unwrap().len(), 5);
    assert!(!modal["custom_id"].as_str().unwrap().contains("marker"));
}
#[test]
fn modal_decoder_rejects_missing_duplicate_and_unknown_fields() {
    let value = json!({"custom_id":"fgfinance:modal:42:3","components":[]});
    let data: serenity::ModalInteractionData = from_value(value).unwrap();
    assert!(modals::values(&data).is_err());
}

#[test]
fn sdk_actor_mapping_denies_public_unknown_and_disabled_contexts() {
    use crate::features::financial_core::OwnerRef;
    use crate::features::identity::{BindingState, DiscordAccountId, OwnerBinding};
    let owner = OwnerBinding::new(
        DiscordAccountId::parse("123").unwrap(),
        OwnerRef::new("synthetic-owner".into()).unwrap(),
        BindingState::Enabled,
    );
    let user: serenity::User = from_value(json!({"id":"123","username":"synthetic"})).unwrap();
    assert!(actor(&user, None, &owner).is_ok());
    assert!(actor(&user, Some(serenity::GuildId::new(1)), &owner).is_err());
    let foreign: serenity::User = from_value(json!({"id":"124","username":"synthetic"})).unwrap();
    assert!(actor(&foreign, None, &owner).is_err());
    let disabled = OwnerBinding::new(
        DiscordAccountId::parse("123").unwrap(),
        OwnerRef::new("synthetic-owner".into()).unwrap(),
        BindingState::Disabled,
    );
    assert!(actor(&user, None, &disabled).is_err());
}

#[test]
fn actual_modal_inputs_require_unique_known_fields_and_private_ack_payloads() {
    let make = |ids: &[&str]| {
        from_value::<serenity::ModalInteractionData>(json!({
        "custom_id":"fgfinance:modal:42:3",
        "components":ids.iter().map(|id|json!({"type":1,"components":[{"type":4,"custom_id":id,"value":"synthetic"}]})).collect::<Vec<_>>()
    })).unwrap()
    };
    assert_eq!(
        modals::values(&make(&["amount", "category", "route", "date", "note"]))
            .unwrap()
            .len(),
        5
    );
    assert!(modals::values(&make(&["amount", "amount", "route", "date", "note"])).is_err());
    assert!(modals::values(&make(&["amount", "category", "route", "date", "unknown"])).is_err());
    let response = to_value(views::private_error("Không có quyền truy cập.".into())).unwrap();
    assert_eq!(response["data"]["flags"].as_u64().unwrap() & 64, 64);
    assert_eq!(response["data"]["allowed_mentions"]["parse"], json!([]));
}

#[tokio::test]
async fn failed_acknowledgement_never_polls_a_financial_action() {
    let called = std::cell::Cell::new(false);
    let result = acknowledged_action(async { Err(JournalError::Delivery) }, async {
        called.set(true);
        Ok(())
    })
    .await;
    assert_eq!(result, Err(JournalError::Delivery));
    assert!(!called.get());
    let result = acknowledged_action(async { Ok(()) }, async {
        called.set(true);
        Err::<(), _>(JournalError::InsufficientFunds)
    })
    .await;
    assert_eq!(result, Ok(Err(JournalError::InsufficientFunds)));
    assert!(called.get());
}
