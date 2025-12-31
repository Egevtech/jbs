pub mod creator;
pub mod parser;

use crate::derive::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub default_target: Option<String>,

    pub variables: Option<HashMap<String, Value>>,
    pub executables: HashMap<String, Executable>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Executable {
    pub name: String,

    pub compiler: String,
    pub linker: String,

    pub compile_options: Option<Vec<String>>,
    pub link_options: Option<Vec<String>>,
    pub libraries: Option<Vec<String>>,

    pub sources: Vec<String>,

    /* User does not need to fill fields below */
    pub executable_canonical_name: Option<String>,
}
