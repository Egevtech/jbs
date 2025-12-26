pub enum PackerError {
    TomlError(toml::ser::Error),
    IOError(std::io::Error),
    InternalError(String),
}