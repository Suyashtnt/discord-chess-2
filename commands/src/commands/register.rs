use error_stack::Result;

use crate::support::{create_cmd_error_reporter, CommandError, Context};

#[poise::command(prefix_command, slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), CommandError> {
    let reporter = create_cmd_error_reporter(vec![], CommandError::from_cmd(&ctx, None));

    reporter.report(
        poise::builtins::register_application_commands_buttons(ctx).await,
        "Could not register commands",
    )?;

    Ok(())
}
