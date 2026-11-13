use std::{
    io,
    sync::{Arc, Mutex},
};
use tracing::{Level, Subscriber};

#[derive(Clone, Default)]
pub struct Writer(Arc<Mutex<Vec<u8>>>);

impl io::Write for Writer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Writer {
    type Writer = Self;
    fn make_writer(&'a self) -> Self {
        self.clone()
    }
}

pub fn assert_filtered<S: Subscriber + Send + Sync>(
    make: impl Fn(Level, Writer) -> S,
    levels: &[Level],
) {
    for &level in levels {
        let writer = Writer::default();
        tracing::subscriber::with_default(make(level, writer.clone()), || {
            tracing::error!(target: "fundguard", "app_error");
            tracing::warn!(target: "fundguard::session", "app_warn");
            tracing::info!(target: "fundguard::session", "app_info");
            tracing::debug!(target: "fundguard::session", "app_debug");
            tracing::trace!(target: "fundguard::session", "app_trace");
            tracing::error!(target: "serenity", "sensitive-sdk-marker");
            tracing::warn!(target: "poise", "sensitive-sdk-marker");
            tracing::info!(target: "reqwest", "sensitive-sdk-marker");
            tracing::debug!(target: "serenity::http", "sensitive-sdk-marker");
            tracing::trace!(target: "serenity::gateway", "sensitive-sdk-marker");
            tracing::error!(target: "fundguard_untrusted", "sensitive-sdk-marker");
        });
        let bytes = writer.0.lock().unwrap();
        let output = String::from_utf8_lossy(&bytes);
        assert!(!output.contains("sensitive-sdk-marker"));
        for (event, threshold) in [
            ("app_error", Level::ERROR),
            ("app_warn", Level::WARN),
            ("app_info", Level::INFO),
            ("app_debug", Level::DEBUG),
            ("app_trace", Level::TRACE),
        ] {
            assert_eq!(
                output.contains(event),
                threshold <= level,
                "{level}: {event}"
            );
        }
    }
}
