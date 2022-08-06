use error_stack::Report;

mod commands;
mod support;
use commands::*;
use poise::Command;

use crate::support::CommandError;

pub fn create_command_list() -> Vec<Command<(), Report<CommandError>>> {
    vec![register(), test_slash(), test_user()]
}
