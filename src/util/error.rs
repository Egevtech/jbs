use std::fmt;
pub enum ArgParseError {
    ParseError(String),
    IdentityPanic(String)
}

impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgParseError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ArgParseError::IdentityPanic(msg) => write!(f, "Identity panic: {}", msg)
        }
    }
}
