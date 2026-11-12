use std::process::ExitCode;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match fundguard::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let fallback = tracing_subscriber::fmt()
                .with_max_level(tracing::Level::ERROR)
                .with_writer(std::io::stderr)
                .finish();
            tracing::subscriber::with_default(fallback, || {
                tracing::error!(event = "application_initialization_failed", error = %error);
            });
            ExitCode::FAILURE
        }
    }
}
