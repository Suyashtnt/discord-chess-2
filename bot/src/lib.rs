use poise::serenity_prelude::SerenityError;
use std::{env::var, fmt::Display};

use error_stack::{Context, IntoReport, Result as ErrRes, ResultExt};
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};
use tokio::task::JoinHandle;
use tracing::info;

#[derive(Debug)]
pub struct BotInitError;

impl Display for BotInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not initialize the bot!")
    }
}

impl Context for BotInitError {}

#[tracing::instrument]
pub fn entrypoint() -> ErrRes<JoinHandle<Result<(), SerenityError>>, BotInitError> {
    info!("initalizing bot...");

    let opts = FrameworkOptions {
        commands: commands::create_command_list(),
        ..Default::default()
    };

    let framework = Framework::builder()
        .token(
            var("DISCORD_TOKEN")
                .into_report()
                .attach_printable("Could not get bot token")
                .change_context(BotInitError)?,
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }))
        .options(opts)
        .intents(serenity::GatewayIntents::non_privileged());

    let handle = tokio::spawn(framework.run());

    info!("Successfully initialised bot");

    Ok(handle)
}
