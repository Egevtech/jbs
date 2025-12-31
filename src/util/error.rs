use std::fmt;
pub enum ArgParseError {
    IdentityPanic(String),
    ProjectPanic(String),
}

impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgParseError::IdentityPanic(msg) => write!(f, "Identity panic: {}", msg),
            ArgParseError::ProjectPanic(msg) => write!(f, "Project panic: {}", msg),
        }
    }
}
