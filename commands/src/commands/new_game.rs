use error_stack::{report, IntoReport, Result, ResultExt};
use poise::serenity_prelude as serenity;
use shakmaty::Color;

use crate::support::{Arg, CommandError, Context};

async fn new_game_logic(
    ctx: &Context<'_>,
    opponent: &serenity::User,
    player_side: Color,
) -> Result<(), CommandError> {
    let cmderr = CommandError::from_cmd(ctx, None);

    ctx.say("Creating game... NOTE: DOES NOTHING RN")
        .await
        .into_report()
        .attach_printable("Could not send user age!")
        .attach(Arg::User("opponent".to_string(), opponent.id))
        .attach(Arg::String("side".to_string(), player_side.to_string()))
        .change_context(cmderr)?;

    Ok(())
}

/// Challenge a user to a chess match
#[poise::command(slash_command)]
pub async fn new_game_slash(
    ctx: Context<'_>,
    #[description = "User to face against"] opponent: serenity::User,
    #[description = "The colour you want to play as. Default is white"] side: Option<String>,
) -> Result<(), CommandError> {
    let side = side.unwrap_or_else(|| "white".to_string());

    let cmderr = CommandError::from_cmd(&ctx, None);

    let color = match &*side.to_lowercase() {
        "b" => Color::Black,
        "black" => Color::Black,
        "w" => Color::White,
        "white" => Color::White,
        _ => {
            return Err(report!(cmderr)
                .attach_printable(format!(
                    "Could not figure out side based on `{}`",
                    side.clone()
                ))
                .attach(Arg::User("opponent".to_string(), opponent.id))
                .attach(Arg::String("side".to_string(), side.to_string())))
        }
    };

    new_game_logic(&ctx, &opponent, color).await
}

/// Challenge a user to a chess match
#[poise::command(context_menu_command = "Test")]
pub async fn new_game_user(ctx: Context<'_>, opponent: serenity::User) -> Result<(), CommandError> {
    new_game_logic(&ctx, &opponent, Color::White).await
}
