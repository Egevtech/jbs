pub mod error;
pub mod parse_commands;

use std::default::Default;

#[derive(PartialEq, Default)]
pub enum CommandType {
    #[default]
    None,

    Build,
    Run,

    Clean,

    Configure,
}

pub struct Command {
    pub command: CommandType,
}

impl Command {
    fn new() -> Command {
        Command {
            command: CommandType::None,
        }
    }
}

