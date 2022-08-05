use poise::Command;
use slash::create_slash_command_list;

mod context;
mod slash;

pub fn create_command_list<U, E>() -> Vec<Command<U, E>> {
    create_slash_command_list()
}
