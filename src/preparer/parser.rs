use std::fmt::{Formatter, Display, Debug};
use std::result::Result as StdResult;
use super::{structs::*};

pub enum ProjectParserError {
    LowLevel(String),
    Parse(String),
}

impl Debug for ProjectParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> StdResult<(), std::fmt::Error> {
        match self {
            ProjectParserError::LowLevel(message) =>
                write!(f, "{}", message),
            ProjectParserError::Parse(message) =>
                write!(f, "{}", message),
        }
    }
}

impl Display for ProjectParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser panicked: {:?}", self)
    }
}

/**
 * Reads file on <path> and loading config
 */
pub fn parse(path: &str) -> StdResult<Project, ProjectParserError> {
    let mut project: Project;

    let contents = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            return Err(ProjectParserError::LowLevel(err.to_string()));
        }
    };

    project = match toml::from_str(&contents) {
        Ok(project) => project,
        Err(err) => {
            return Err(ProjectParserError::Parse(err.to_string()))
        }
    };

    Ok(project)
}

impl Executable {

}
