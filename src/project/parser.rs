use super::*;
use std::fmt::{Debug, Display, Formatter};
use std::result::Result as StdResult;

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

/**
Reads file on `path`  and loading config
```rust
let project = parse("project.toml");
match project {
    ...
}
```
 */
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
