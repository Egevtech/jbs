pub mod parse_commands;
pub mod error;

use std::collections::HashMap;

#[derive(PartialEq)]
pub enum CommandType {
    Build,
    Run,

    Clean,

    Configure,

    None,
}

pub type Derives = HashMap<String, String>;

pub struct Command {
    pub command: CommandType,
    pub args: Vec<String>,

    pub derives: Derives,
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