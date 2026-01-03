use crate::packer::*;
use crate::project::Executable;

pub fn pack_executable(mut executable: Executable, derives: Derives) -> Result<(), PackerError> {
    if executable.executable_canonical_name.is_none() {
        return Err(PackerError::Internal(
            "system field is empty, but it cant be empty".to_owned(),
        ));
    }

    let result = match toml::to_string(&executable.as_packed(derives)) {
        Ok(string) => string,
        Err(error) => return Err(PackerError::Toml(error)),
    };

    if let Err(error) = std::fs::write(
        format!(
            "build/fixed/{}.toml",
            executable.executable_canonical_name.unwrap()
        ),
        result,
    ) {
        return Err(PackerError::IO(error));
    }

    Ok(())
}
