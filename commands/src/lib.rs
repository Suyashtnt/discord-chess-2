use error_stack::Report;

mod commands;
mod support;
use commands::*;
use poise::Command;

pub use support::Arg;
pub use support::CommandError;
pub fn create_command_list() -> Vec<Command<(), Report<CommandError>>> {
    vec![
        register(),
        test_slash(),
        test_user(),
        new_game_slash(),
        new_game_user(),
    ]
}
