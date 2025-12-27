use std::process::exit;
use crate::packer::{pack_executable::pack_executable};
use crate::project::{creator::prepare_dirs};
use crate::project::{Project, Executable};

pub fn configure_project(project: &Project) {
    if let Some(e) = prepare_dirs() {
        eprintln!("Failed to prepare project directories: {}", e);
        exit(1);
    }

    for (index, source) in project.executables.clone().into_iter().enumerate() {
        println!("[{}%] Packing executable {}...", (index)*100/project.executables.len(), source.1.name);
        let (canonical_name, mut exec) = (source.0.clone(), (source.1.clone()));

        exec.executable_canonical_name = Some(canonical_name);

        if let Err(e) = pack_executable(exec.clone()) {
            eprintln!("Failed to pack executable \"{}\", panic: {}", exec.name, e);
        }

        println!("[{}%] Packed executable {}/{}",
                 (index+1)*100/project.executables.len(), index+1, project.executables.len());

    }
}