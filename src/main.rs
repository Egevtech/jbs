mod operator;
mod preparer;
mod packer;
use preparer::{parser::*, structs::*};

struct ExecutableLock {
    name: String,

    compiler: String,
    linker: String,

    compile_flags: Vec<String>,
    link_flags: Vec<String>,
}
    
impl ExecutableLock {
    fn new() -> ExecutableLock {
            ExecutableLock {
            name: "".to_string(),
            compiler: "".to_string(),
            linker: "".to_string(),
    
            compile_flags: vec![],
            link_flags: vec![]
        }
    }
}


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