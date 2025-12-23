use toml::*;

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    version: String,

    variables: HashMap<String, String>,
    executables: HashMap<String, Executable>
}

#[derive(Debug, Deserialize)]
struct Executable {
    name: String,

    compiler: String,
    linker: String,

    compile_options: Vec<String>,
    link_options: Vec<String>,
    libraries: Vec<String>,

    sources: Vec<String>
}

fn main() {
}
