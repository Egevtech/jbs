use crate::derive::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::result::Result as StdResult;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,

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

pub fn prepare_dirs() -> Option<std::io::Error> {
    let dirs = vec!["./build", "./build/cache", "./build/fixed"];

    for dir in dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            return Some(e);
        }
    }

    None
}

pub fn clean() -> Option<std::io::Error> {
    if let Err(e) = fs::remove_dir_all("build/") {
        return Some(e);
    }

    None
}

pub enum ProjectParserError {
    LowLevel(String),
    Parse(String),
}

impl Debug for ProjectParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdResult<(), std::fmt::Error> {
        match self {
            ProjectParserError::LowLevel(message) => write!(f, "{}", message),
            ProjectParserError::Parse(message) => write!(f, "{}", message),
        }
    }
}

impl Display for ProjectParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser panicked: {:?}", self)
    }
}

pub fn parse(path: &str) -> StdResult<Project, ProjectParserError> {
    let contents = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            return Err(ProjectParserError::LowLevel(err.to_string()));
        }
    };

    let project: Project = match toml::from_str(&contents) {
        Ok(project) => project,
        Err(err) => return Err(ProjectParserError::Parse(err.to_string())),
    };

    Ok(project)
}

impl Executable {}
