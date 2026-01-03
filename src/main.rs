mod derive;
mod operator;
mod packer;
mod project;
mod util;

use crate::operator::builder::*;
use crate::operator::configurator::configure_project;
use crate::packer::unpack_executable::*;
use project::*;
use util::{parse_commands::*, *};

fn main() {
    let mut project: Project;

    match std::fs::exists("project.toml") {
        Ok(b) => {
            if !b {
                eprintln!("project.toml does not exist");
                return;
            }
        }
        Err(e) => {
            eprintln!("project.toml does not exist: {}", e);
            return;
        }
    }

    project = match parse("project.toml") {
        Ok(project) => project,
        Err(err) => {
            println!("Parse error: {}", err);
            return;
        }
    };

    let command: Command =
        match parse_commands(std::env::args().collect::<Vec<String>>(), &mut project) {
            Ok(command) => command,
            Err(err) => {
                println!("Argument parse error: {}", err);
                return;
            }
        };

    match command.command {
        CommandType::Clean => {
            if let Some(err) = clean() {
                eprintln!("Failed to clean: {err}");
            }
        }
        CommandType::Configure => configure_project(&project),
        CommandType::None => println!("Nothing to do"),
        CommandType::Build => {
            build_project(&project);
            drop(project);
            std::process::exit(0);
        }
        CommandType::Run => {
            build_project(&project);
            println!("Running executable...\n");

            let _ =
                std::process::Command::new(format!("build/{}", project.default_target.unwrap()))
                    .spawn()
                    .expect("Cant spawn process");
            std::process::exit(0);
        }
    }
}

fn build_project(project: &Project) {
    let file = format!(
        "build/fixed/{}.toml",
        project.default_target.clone().unwrap()
    );
    println!("Parsing executable lock file: {file}");
    let target = unpack_executable(file.to_string()).expect("Unpack failed");

    build_executable(target, project.name.clone());
}
