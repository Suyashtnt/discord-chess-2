use error_stack::{IntoReport, Result, ResultExt};
use poise::serenity_prelude::{self as serenity, Timestamp};

use crate::support::{Arg, CommandError, Context};

pub fn get_user_age_msg(username: &str, user_age: &Timestamp) -> String {
    format!("{}'s account was created at {}", username, user_age)
}

async fn test_cmd_logic(ctx: &Context<'_>, user: &serenity::User) -> Result<(), CommandError> {
    let response = get_user_age_msg(&user.name, &user.created_at());

    ctx.say(response)
        .await
        .into_report()
        .attach_printable("Could not send user age!")
        .attach(Arg::User("user".to_string(), user.id))
        .change_context(CommandError::from_cmd(&ctx, None))?;

    Ok(())
}

/// Simple test command that displays when an account was created
#[poise::command(slash_command)]
pub async fn test_slash(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), CommandError> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    test_cmd_logic(&ctx, u).await
}

/// Simple test command that displays when an account was created
#[poise::command(context_menu_command = "Test")]
pub async fn test_user(ctx: Context<'_>, user: serenity::User) -> Result<(), CommandError> {
    test_cmd_logic(&ctx, &user).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_create_string() {
        let timestamp =
            Timestamp::parse("2019-02-10T16:34:35.279Z").expect("Could not parse timestamp");

        let expected = "Tabiasgee Human's account was created at 2019-02-10T16:34:35.279Z";
        let result = get_user_age_msg("Tabiasgee Human", &timestamp);

        assert_eq!(expected, result);
    }
}
