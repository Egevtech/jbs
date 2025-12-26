use crate::preparer::structs::{Executable};
use serde::Serialize;
use crate::packer::errors::PackerError;

fn pack_executable(executable: Executable) -> Result<(), super::errors::PackerError> {

    if executable.executable_canonical_name == None {
        return Err(PackerError::InternalError("\
            system field is empty, but it cant be empty\
            ".to_owned()));
    }

    let result = match toml::to_string(&executable) {
        Ok(string) => string,
        Err(error) => return Err(PackerError::TomlError(error))
    };

    if let Err(error) =
        std::fs::write(format!("build/fixed/{}.toml",
                                    executable.executable_canonical_name.unwrap()),
                       result) {
        return Err(PackerError::IOError(error))
    }

    Ok(())
}