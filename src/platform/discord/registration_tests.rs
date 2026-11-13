use super::*;
use std::cell::Cell;

#[tokio::test(start_paused = true)]
async fn named_upsert_has_exact_scope_builder_and_one_attempt() {
    let calls = &Cell::new(0);
    register(
        serenity::GuildId::new(12),
        crate::features::identity::presentation::fundguard(),
        tokio::time::Instant::now() + std::time::Duration::from_secs(30),
        |guild, command| async move {
            calls.set(calls.get() + 1);
            assert_eq!(guild.get(), 12);
            let value = serenity::json::to_value(command).unwrap();
            assert_eq!(value["name"], "fundguard");
            assert_eq!(value["type"], 1);
            assert!(
                value["options"]
                    .as_array()
                    .is_none_or(|options| options.is_empty())
            );
            Ok::<_, ()>(())
        },
    )
    .await
    .unwrap();
    assert_eq!(calls.get(), 1);
    for seconds in [3, 30] {
        let start = tokio::time::Instant::now();
        let result = register(
            serenity::GuildId::new(12),
            crate::features::identity::presentation::fundguard(),
            start + std::time::Duration::from_secs(seconds),
            |_, _| std::future::pending::<Result<(), ()>>(),
        )
        .await;
        assert_eq!(result.unwrap_err(), SessionError::RegistrationTimeout);
        assert_eq!(start.elapsed().as_secs(), seconds.min(10));
    }
    assert_eq!(
        register(
            serenity::GuildId::new(12),
            crate::features::identity::presentation::fundguard(),
            tokio::time::Instant::now() + std::time::Duration::from_secs(30),
            |_, _| async { Err::<(), _>("provider-marker") }
        )
        .await
        .unwrap_err(),
        SessionError::Registration
    );
}
