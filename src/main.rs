use error_stack::{Context, IntoReport, Result, ResultExt};
use std::fmt;
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug)]
struct StartupError;

impl fmt::Display for StartupError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Failed to init chess bot")
    }
}
impl Context for StartupError {}

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(EnvFilter::from_default_env());

    tracing_subscriber::registry().with(fmt_layer).init();

    state::entrypoint().change_context(StartupError)?;
    database::entrypoint().await.change_context(StartupError)?;
    let handle = bot::entrypoint().change_context(StartupError)?;

    handle
        .await
        .into_report()
        .change_context(StartupError)?
        .into_report()
        .attach_printable("Failed to start bot")
        .change_context(StartupError)?;
    Ok(())
}
