pub mod parse_commands;
pub mod error;

use std::collections::HashMap;

pub enum CommandType {
    Build,
    Run,

    Clean,

    Configure,

    None,
}

pub type Derives = HashMap<String, String>;

pub struct Command {
    command: CommandType,
    args: Vec<String>,

    derives: Derives,
}

impl Command {
    fn new() -> Command {
       Command {
           command: CommandType::None,

           args: Vec::new(),
           derives: HashMap::new(),
       }
    }
}