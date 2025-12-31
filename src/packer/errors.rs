use std::fmt;

pub enum PackerError {
    Toml(toml::ser::Error),
    IO(std::io::Error),
    Internal(String),
}

impl fmt::Display for PackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackerError::Toml(e) => write!(f, "Toml failed: {}", e),
            PackerError::IO(e) => write!(f, "IO failed: {}", e),
            PackerError::Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}
