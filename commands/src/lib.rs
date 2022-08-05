use std::fmt::Display;

use error_stack::{Context as ErrContext, IntoReport, Report, ResultExt};
use poise::{self, serenity_prelude as serenity, Command};
use slash::create_slash_command_list;

mod context;
mod slash;

#[derive(Debug)]
pub enum Arg {
    String(String, String),
    User(String, serenity::UserId),
    Int(String, i64),
    Number(String, f64),
    Boolean(String, bool),
    Channel(String, serenity::ChannelId),
    Role(String, serenity::RoleId),
    Mentionable(String, serenity::Mention),
    Attachment(String, serenity::Attachment),
}

#[derive(Debug)]
pub struct CommandError {
    pub name: String,
    pub runner: serenity::UserId,
    pub guild: Option<serenity::GuildId>,
    pub channel: serenity::ChannelId,
    pub args: Vec<Arg>,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to run command {}!", self.name)
    }
}

impl CommandError {
    pub fn from_ctx(ctx: &Context<'_>, args: Vec<Arg>) -> Self {
        Self {
            name: ctx.command().name.clone(),
            runner: ctx.author().id,
            guild: ctx.guild_id(),
            channel: ctx.channel_id(),
            args,
        }
    }
}

impl ErrContext for CommandError {}

pub type Context<'a> = poise::Context<'a, (), Report<CommandError>>;

#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Report<CommandError>> {
    poise::builtins::register_application_commands_buttons(ctx)
        .await
        .report()
        .attach_printable("Could not do registration")
        .change_context(CommandError::from_ctx(&ctx, vec![]))?;

    Ok(())
}

pub fn create_command_list() -> Vec<Command<(), Report<CommandError>>> {
    let mut vec = vec![];

    vec.append(&mut create_slash_command_list());
    vec.append(&mut vec![register()]);

    vec
}
