#[path = "../../tests/support/logging.rs"]
mod logging;

#[test]
fn sdk_targets_are_suppressed_at_every_level() {
    logging::assert_filtered(
        super::subscriber,
        &[
            tracing::Level::ERROR,
            tracing::Level::WARN,
            tracing::Level::INFO,
            tracing::Level::DEBUG,
            tracing::Level::TRACE,
        ],
    );
}
