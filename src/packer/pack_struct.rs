use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
struct Executable {
    pub name: String,
    pub title: String,

    pub compiler: String,
    pub linker: String,

    pub compile_options: Vec<String>,
    pub link_options: Vec<String>
}