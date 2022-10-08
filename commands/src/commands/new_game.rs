use error_stack::{Context as ErrContext, Result};
use poise::{
    futures_util::TryFutureExt,
    serenity_prelude::{self as serenity, ButtonStyle, Mentionable},
};
use shakmaty::Color;
use std::fmt::Display;

use crate::support::{create_cmd_error_reporter, Arg, CommandError, Context};

pub enum NGChallengeResult {
    Accept,
    Deny,
}

impl Display for NGChallengeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Accept => "ng_challenge_accept",
            Self::Deny => "ng_challenge_deny",
        })
    }
}

#[derive(Debug)]
pub enum NewGameErr {
    ChallengeVariantError(String),
    SideParseError(String),
}

impl Display for NewGameErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            NewGameErr::ChallengeVariantError(v) => {
                format!("Could not parse challenge variant {v}")
            }
            NewGameErr::SideParseError(v) => format!("Could not parse side variant {v}"),
        })
    }
}

impl ErrContext for NewGameErr {}

impl TryFrom<String> for NGChallengeResult {
    type Error = NewGameErr;

    fn try_from(value: String) -> std::result::Result<NGChallengeResult, NewGameErr> {
        match &*value {
            "ng_challenge_accept" => Ok(Self::Accept),
            "ng_challenge_deny" => Ok(Self::Deny),
            _ => Err(NewGameErr::ChallengeVariantError(value)),
        }
    }
}

async fn new_game_logic<'a>(
    ctx: &Context<'a>,
    opponent: &serenity::User,
    player_side: Color,
) -> Result<(), CommandError> {
    let author = ctx.author();
    let reporter = create_cmd_error_reporter(
        vec![
            Arg::User("opponent".to_string(), opponent.id),
            Arg::String("side".to_string(), player_side.to_string()),
        ],
        CommandError::from_cmd(ctx, None),
    );

    let mut msg = reporter.clone().report(
        ctx.send(|m| {
            m.content(format!("{}", opponent.mention()))
                .embed(|e| {
                    e.title("A challenger approaches!").description(format!(
                        "{} wants to face against {}! Do you accept {}?",
                        author.mention(),
                        opponent.mention(),
                        opponent.mention()
                    ))
                })
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_button(|b| {
                            b.label("Accept")
                                .style(ButtonStyle::Success)
                                .custom_id(NGChallengeResult::Accept)
                        })
                        .create_button(|b| {
                            b.label("Deny")
                                .style(ButtonStyle::Danger)
                                .custom_id(NGChallengeResult::Deny)
                        })
                    })
                })
        })
        .and_then(|v| v.into_message()) // ::message causes borrowing errors
        .await,
        "Could not send message",
    )?;

    let interaction = msg.await_component_interaction(ctx.discord()).await;

    match interaction {
        Some(interaction) => {
            match reporter.clone().report(
                interaction.data.custom_id.clone().try_into(),
                "Could not convert custom ID",
            )? {
                NGChallengeResult::Accept => {
                    reporter.report(
                        msg.edit(ctx.discord(), |e| {
                            e.content("Game accepted! Creating board...")
                        })
                        .await,
                        "Could not edit response",
                    )?;
                }
                NGChallengeResult::Deny => {
                    reporter.report(
                        msg.edit(ctx.discord(), |e| {
                            e.content("Game cancelled due to opponent denying")
                        })
                        .await,
                        "Could not edit response",
                    )?;
                }
            }
        }
        None => {
            reporter.report(
                msg.edit(ctx.discord(), |e| {
                    e.content("Game cancelled due to timeout")
                })
                .await,
                "Could not edit response",
            )?;
        }
    }

    Ok(())
}

/// Challenge a user to a chess match
#[poise::command(slash_command)]
pub async fn new_game(
    ctx: Context<'_>,
    #[description = "User to face against"] opponent: serenity::User,
    #[description = "The colour you want to play as. Default is white"] side: Option<String>,
) -> Result<(), CommandError> {
    let side = side.unwrap_or_else(|| "white".to_string());

    let reporter = create_cmd_error_reporter(
        vec![
            Arg::User("opponent".to_string(), opponent.id),
            Arg::String("side".to_string(), side.to_string()),
        ],
        CommandError::from_cmd(&ctx, None),
    );

    let color = match &*side.to_lowercase() {
        "b" => Color::Black,
        "black" => Color::Black,
        "w" => Color::White,
        "white" => Color::White,
        _ => {
            return reporter.report(
                Err(NewGameErr::SideParseError(side.clone())),
                format!("Could not figure out side based on `{}`", side.clone()),
            )
        }
    };

    new_game_logic(&ctx, &opponent, color).await
}

/// Challenge a user to a chess match
#[poise::command(context_menu_command = "Challenge")]
pub async fn new_game_user(ctx: Context<'_>, opponent: serenity::User) -> Result<(), CommandError> {
    new_game_logic(&ctx, &opponent, Color::White).await
}
