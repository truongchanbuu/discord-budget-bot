use super::*;

#[test]
fn errors_are_safe_and_preserve_reporting_categories() {
    for (error, event) in [
        (
            ApplicationError::configuration("FUNDGUARD_MODE"),
            "application_initialization_failed",
        ),
        (
            ApplicationError::session("startup_timeout"),
            "discord_session_failed",
        ),
    ] {
        assert_eq!(error.event_name(), event);
        let text = format!("{error} {error:?} {error:#?}");
        assert!(!text.contains("sensitive-provider-marker"));
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[tokio::test]
async fn dispatch_validates_before_initialization_and_connection() {
    use std::cell::RefCell;
    let events = RefCell::new(Vec::new());
    let mut lookup = |name: &str| match name {
        "FUNDGUARD_MODE" => Some("discord".into()),
        "FUNDGUARD_DISCORD_TOKEN" => Some("synthetic-token-marker".into()),
        "FUNDGUARD_DISCORD_GUILD_ID" => Some("12".into()),
        "FUNDGUARD_DISCORD_OWNER_ID" => Some("34".into()),
        "FUNDGUARD_OWNER_REF" => Some("owner-marker".into()),
        "FUNDGUARD_OWNER_STATE" => Some("disabled".into()),
        _ => None,
    };
    dispatch(
        &mut lookup,
        || {
            events.borrow_mut().push("initialize");
            Ok(())
        },
        |_| async {
            events.borrow_mut().push("connect");
            Ok(())
        },
    )
    .await
    .unwrap();
    assert_eq!(*events.borrow(), ["initialize", "connect"]);
    events.borrow_mut().clear();
    assert!(
        dispatch(
            &mut |_| Some("invalid".into()),
            || {
                events.borrow_mut().push("initialize");
                Ok(())
            },
            |_| async {
                events.borrow_mut().push("connect");
                Ok(())
            }
        )
        .await
        .is_err()
    );
    assert!(events.borrow().is_empty());
    dispatch(
        &mut |_| None,
        || {
            events.borrow_mut().push("initialize");
            Ok(())
        },
        |_| async { panic!("offline must not construct a client") },
    )
    .await
    .unwrap();
    assert_eq!(*events.borrow(), ["initialize"]);
}
