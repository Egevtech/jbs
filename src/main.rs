mod operator;
mod project;
mod packer;
mod util;

use project::{parser::*, *};

fn main() {
    let mut project: Project;

    project = match parse("project.toml") {
        Ok(project) => project,
        Err(err) => {
            println!("Parse error: {}", err);
            return
        }
    }
}