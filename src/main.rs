mod derive;
mod operator;
mod packer;
mod project;

use crate::operator::builder::*;
use crate::operator::configurator::configure_project;
use crate::packer::unpack_executable::*;
use project::*;

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
