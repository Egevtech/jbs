pub mod parse_commands;

use std::default::Default;
use std::fmt;

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

pub enum ArgParseError {
    IdentityPanic(String),
    ProjectPanic(String),
}

impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgParseError::IdentityPanic(msg) => write!(f, "Identity panic: {}", msg),
            ArgParseError::ProjectPanic(msg) => write!(f, "Project panic: {}", msg),
        }
    }
}
