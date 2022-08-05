use crate::{Arg, CommandError, Context};
use error_stack::{IntoReport, Result, ResultExt};
use poise::serenity_prelude as serenity;

/// Simple test command that displays when an account was created
#[poise::command(slash_command, prefix_command)]
pub async fn test(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), CommandError> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());

    ctx.say(response)
        .await
        .report()
        .attach_printable("Could not send user age!")
        .change_context(CommandError::from_ctx(
            &ctx,
            vec![Arg::User("user".to_string(), u.id)],
        ))?;

    Ok(())
}
