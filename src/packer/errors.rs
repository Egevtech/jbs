use std::fmt;

pub enum PackerError {
    TomlError(toml::ser::Error),
    IOError(std::io::Error),
    InternalError(String),
}

impl fmt::Display for PackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackerError::TomlError(e) => write!(f, "Toml failed: {}", e),
            PackerError::IOError(e) => write!(f, "IO failed: {}", e),
            PackerError::InternalError(e) => write!(f, "Internal error: {}", e),
        }
    }
}