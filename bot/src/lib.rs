use std::env::var;

use anyhow::Context;
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions, FrameworkBuilder};
use tracing::info;

pub fn entrypoint() -> anyhow::Result<()> {
    info!("initalizing bot...");

    let opts = FrameworkOptions {
        commands: commands::create_command_list(),
        ..Default::default()
    };

    let framework: FrameworkBuilder<(), anyhow::Error> = Framework::build()
        .token(var("TOKEN").context("Could not get bot token")?)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }))
        .options(opts)
        .intents(serenity::GatewayIntents::non_privileged());

    tokio::spawn(framework.run());
    
    info!("Successfully initialised bot");

    Ok(())
}

