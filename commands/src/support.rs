use error_stack::{Context as ErrContext, IntoReport, Report, ResultExt};
use poise::{self, serenity_prelude as serenity};
use std::fmt::Display;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
/// An error happened when running a command
pub struct CommandError {
    /// The command name
    pub name: String,
    /// The runner of the command
    pub runner: serenity::UserId,
    /// the guild the command was ran in
    pub guild: Option<serenity::GuildId>,
    /// The channel the command was ran in
    pub channel: serenity::ChannelId,
    /// The game ID if this command had a game
    pub game_id: Option<String>,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to run command {}!", self.name)
    }
}

impl CommandError {
    pub fn from_cmd(ctx: &Context<'_>, game_id: Option<String>) -> Self {
        Self {
            name: ctx.command().name.clone(),
            runner: ctx.author().id,
            guild: ctx.guild_id(),
            channel: ctx.channel_id(),
            game_id,
        }
    }
}

impl ErrContext for CommandError {}

pub type Context<'a> = poise::Context<'a, (), Report<CommandError>>;

#[derive(Clone)]
pub struct CmdErrorReporter {
    args: Vec<Arg>,
    cmd_err: CommandError,
}

impl CmdErrorReporter
{
    pub fn report<V, E, S>(self, err: Result<V, E>, reason: S) -> Result<V, Report<CommandError>>
    where
        Report<E>: From<E>,
        S: ToString
    {
        let mut new_err = err.into_report();
        for arg in self.args {
            new_err = new_err.attach(arg);
        }
        new_err
            .attach_printable(reason.to_string())
            .change_context(self.cmd_err)
    }
}

pub fn create_cmd_error_reporter(
    args: Vec<Arg>,
    cmd_err: CommandError,
) -> CmdErrorReporter
{
    CmdErrorReporter { args, cmd_err }
}
