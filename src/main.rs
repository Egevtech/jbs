mod operator;
mod project;
mod packer;
mod util;

use project::{parser::*, *};
use util::{parse_commands::*, error::*, *};
use crate::operator::configurator::configure_project;

fn main() {
    let mut project: Project;

    match std::fs::exists("project.toml") {
    project = match parse("project.toml") {
        Ok(project) => project,
        Err(err) => {
            println!("Parse error: {}", err);
            return
        }
    };

    let command: Command;
    command = match parse_commands(std::env::args().collect::<Vec<String>>(), &mut project) {
        Ok(command) => command,
        Err(err) => {
            println!("Argument parse error: {}", err);
            return
        }
    };

    match command.command {
        CommandType::Clean => {
            if let Err(e) = std::fs::remove_dir("./build") {
                println!("Clean failed: {}", e);
            }
        },
        CommandType::Configure => configure_project(&project),
        CommandType::None => println!("Nothing to do"),
        CommandType::Build => {},
        CommandType::Run => {},
    }
}