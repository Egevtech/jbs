use crate::project::Project;
use crate::util::{Command, CommandType, *};

pub fn parse_commands(args: Vec<String>, project: &mut Project) -> Result<Command, ArgParseError> {
    let mut result: Command = Command::new();

    result.command = CommandType::default();

    let mut i = 1;

    while i < args.len() {
        let arg = args[i].clone();

        let check_ident = || -> Result<(), ArgParseError> {
            if result.command != CommandType::None {
                return Err(ArgParseError::IdentityPanic(
                    "Only one operation per run is allowed".to_string(),
                ));
            }

            Ok(())
        };

        match arg.as_str() {
            "build" => {
                check_ident()?;

                result.command = CommandType::Build;

                if project.default_target.is_none() && args.len() - 1 <= i {
                    return Err(ArgParseError::ProjectPanic(
                        "No target specified".to_string(),
                    ));
                }

                if project.default_target.is_none() {
                    project.default_target = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "run" => {
                check_ident()?;

                result.command = CommandType::Run;

                if project.default_target.is_none() && args.len() - 1 <= i {
                    return Err(ArgParseError::ProjectPanic(
                        "No target specified".to_string(),
                    ));
                }

                if project.default_target.is_none() {
                    project.default_target = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "configure" => {
                check_ident()?;

                result.command = CommandType::Configure;
            }
            "clean" => {
                check_ident()?;

                result.command = CommandType::Clean;
            }
            _ => {
                eprintln!("Unknown command \"{}\"", arg);
            }
        }
        i += 1;
    }

    Ok(result)
}
