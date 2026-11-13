use std::process::ExitCode;
use tracing_subscriber::{Layer, layer::SubscriberExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match fundguard::start().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let fallback = fallback_subscriber(std::io::stderr);
            tracing::subscriber::with_default(fallback, || {
                tracing::error!(event = error.event_name(), error = %error);
            });
            ExitCode::FAILURE
        }
    }
}

fn fallback_subscriber<W>(writer: W) -> impl tracing::Subscriber + Send + Sync
where
    W: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
{
    tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(writer)
            .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
                *metadata.level() <= tracing::Level::ERROR
                    && (metadata.target() == "fundguard"
                        || metadata.target().starts_with("fundguard::"))
            })),
    )
}

#[cfg(test)]
#[path = "../tests/support/logging.rs"]
mod logging;

#[cfg(test)]
#[test]
fn fallback_suppresses_sdk_targets() {
    logging::assert_filtered(
        |_, writer| fallback_subscriber(writer),
        &[tracing::Level::ERROR],
    );
}
