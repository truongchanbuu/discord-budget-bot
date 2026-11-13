use super::*;
use crate::features::identity::BindingState;
use ::serenity::json::{from_value, json, to_value};
use std::sync::atomic::Ordering;

pub(crate) fn fixture(user: &str, guild: Option<&str>) -> serenity::CommandInteraction {
    from_value(json!({
        "id":"100", "application_id":"101", "data":{"id":"102","name":"fundguard","type":1},
        "guild_id":guild, "channel_id":"103", "user":{"id":user,"username":"spoofed-owner"},
        "token":"synthetic-interaction-marker", "version":1, "locale":"en-US", "entitlements":[],
        "attachment_size_limit":100
    }))
    .unwrap()
}

fn data(state: BindingState) -> CommandData {
    CommandData::new(
        OwnerBinding::new(
            DiscordAccountId::parse("34").unwrap(),
            crate::features::financial_core::OwnerRef::new("owner-marker".into()).unwrap(),
            state,
        ),
        serenity::GuildId::new(12),
    )
}

#[test]
fn authentic_actor_context_and_current_policy_determine_readiness() {
    let data = data(BindingState::Enabled);
    assert_eq!(
        decide(&data, &fixture("34", Some("12"))),
        ReadinessDecision::Ready
    );
    for interaction in [
        fixture("35", Some("12")),
        fixture("34", Some("13")),
        fixture("34", None),
    ] {
        assert_eq!(decide(&data, &interaction), ReadinessDecision::Denied);
    }
    let mut missing = fixture("34", Some("12"));
    missing.user = serenity::User::default();
    assert_eq!(decide(&data, &missing), ReadinessDecision::Denied);
    data.accepting.store(false, Ordering::Release);
    assert_eq!(
        decide(&data, &fixture("34", Some("12"))),
        ReadinessDecision::Denied
    );
    let disabled = self::data(BindingState::Disabled);
    assert_eq!(
        decide(&disabled, &fixture("34", Some("12"))),
        ReadinessDecision::Denied
    );
    assert!(!format!("{disabled:#?}").contains("marker"));
}

#[test]
fn actual_command_and_reply_builders_are_minimal_and_private() {
    let command = fundguard();
    assert!(command.ephemeral);
    let command = to_value(command.create_as_slash_command().unwrap()).unwrap();
    assert_eq!(command["name"], "fundguard");
    assert!(
        command["options"]
            .as_array()
            .is_none_or(|values| values.is_empty())
    );
    for (decision, text) in [
        (
            ReadinessDecision::Ready,
            "FundGuard is ready. No financial operations are available.",
        ),
        (ReadinessDecision::Denied, "Access denied."),
    ] {
        let value = to_value(
            reply(decision)
                .to_slash_initial_response(serenity::CreateInteractionResponseMessage::new()),
        )
        .unwrap();
        assert_eq!(value["content"], text);
        assert_eq!(value["flags"], 64);
        assert_eq!(value["allowed_mentions"]["parse"], json!([]));
    }
}

#[tokio::test(start_paused = true)]
async fn failed_duplicate_and_timed_out_sends_are_not_retried() {
    use std::cell::Cell;
    let calls = Cell::new(0);
    for _ in 0..2 {
        let result = send_private(ReadinessDecision::Ready, |_| async {
            calls.set(calls.get() + 1);
            Err::<(), _>("synthetic duplicate-ack provider marker")
        })
        .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "private response unavailable"
        );
    }
    assert_eq!(calls.get(), 2);
    let started = tokio::time::Instant::now();
    assert!(
        send_private(ReadinessDecision::Denied, |_| std::future::pending::<
            Result<(), ()>,
        >())
        .await
        .is_err()
    );
    assert_eq!(started.elapsed(), std::time::Duration::from_secs(2));
}
