use super::*;
use std::error::Error;

pub fn unpack_executable(source: String) -> Result<ExecutablePack, Box<dyn Error>> {
    let contents = match std::fs::read_to_string(source) {
        Ok(c) => c,
        Err(err) => return Err(Box::new(err)),
    };

    let result: ExecutablePack = match toml::from_str(contents.as_str()) {
        Ok(t) => t,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(result)
}
