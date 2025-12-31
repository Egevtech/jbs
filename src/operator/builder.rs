use crate::packer::ExecutablePack;
use std::{
    io::Write,
    process::{Command, Output},
};

/*
* Code 10 - compiler panic,
* Code 11 - linker panic,
*/

pub fn build_executable(executable: ExecutablePack) {
    let start_time = std::time::Instant::now();
    for (index, source) in executable.sources.iter().enumerate() {
        println!(
            "[{}%] Compiling {}...",
            index * 100 / executable.sources.len(),
            source
        );

        if compile(
            executable.compiler.clone(),
            source.to_string(),
            executable.compile_options.clone(),
        ) {
            eprintln!("Exit due to compiler panic");
            std::process::exit(10);
        }

        println!(
            "[{}%] Compiled {}",
            (index + 1) * 100 / executable.sources.len(),
            source
        );
    }
    println!("Linking executable {}...", executable.name);
    if link(executable.linker, executable.link_options) {
        eprintln!("Exit due to linker panic");
        std::process::exit(11);
    }
    println!("Linking finished");

    println!("Build finished in {}s", start_time.elapsed().as_secs_f32());
}

fn compile(compiler: String, source: String, mut args: Vec<String>) -> bool {
    args.extend(vec![
        "-c".to_string(),
        source.clone(),
        "-o".to_string(),
        format!("build/cache/{}.o", source.clone()),
    ]);
    let output = Command::new(compiler)
        .args(args)
        .output()
        .expect("Failed to run compiler");

    print_output(output)
}

fn link(linker: String, args: Vec<String>) -> bool {
    let output = Command::new(linker)
        .args(args)
        .output()
        .expect("Failed to run linker");

    print_output(output)
}

fn print_output(output: Output) -> bool {
    if !output.stdout.is_empty() {
        let _ = std::io::stdout().write_all(&output.stdout);
    }
    if !output.stderr.is_empty() {
        let _ = std::io::stderr().write_all(&output.stderr);
        println!(
            "Compilation aborted, compiler exited with status {}",
            output.status
        );
        return true;
    }

    false
}
