use error_stack::{IntoReport, Result, ResultExt};

use crate::support::{CommandError, Context};

#[poise::command(prefix_command, slash_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), CommandError> {
    poise::builtins::register_application_commands_buttons(ctx)
        .await
        .report()
        .attach_printable("Could not do registration")
        .change_context(CommandError::from_ctx(&ctx, vec![], None))?;

    Ok(())
}
