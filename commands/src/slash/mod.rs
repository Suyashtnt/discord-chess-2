mod test;

use error_stack::Report;
use poise::Command;

use crate::CommandError;

use test::*;

pub fn create_slash_command_list() -> Vec<Command<(), Report<CommandError>>> {
    vec![test()]
}
