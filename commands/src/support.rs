use error_stack::{Context as ErrContext, Report};
use poise::{self, serenity_prelude as serenity};
use std::fmt::Display;

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
    pub game_id: Option<String>,
    pub args: Vec<Arg>,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to run command {}!", self.name)
    }
}

impl CommandError {
    pub fn from_ctx(ctx: &Context<'_>, args: Vec<Arg>, game_id: Option<String>) -> Self {
        Self {
            name: ctx.command().name.clone(),
            runner: ctx.author().id,
            guild: ctx.guild_id(),
            channel: ctx.channel_id(),
            game_id,
            args,
        }
    }
}

impl ErrContext for CommandError {}

pub type Context<'a> = poise::Context<'a, (), Report<CommandError>>;
