mod operator;
mod project;
mod packer;
mod util;

use project::{parser::*, *};
use util::{parse_commands::*, error::*, *};

fn main() {
    let mut project: Project;

    project = match parse("project.toml") {
        Ok(project) => project,
        Err(err) => {
            println!("Parse error: {}", err);
            return
        }
    }

    let command: Command;
    command = parse_command(project);
}