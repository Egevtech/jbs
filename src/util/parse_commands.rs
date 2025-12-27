use std::cmp::PartialEq;
use std::process::exit;
use crate::util::{Command, CommandType, error::*};
use crate::project::Project;
use crate::operator::{*, configurator::configure_project};

pub fn parse_commands(args: Vec<String>, project: &mut Project) -> Result<Command, ArgParseError> {
    let mut result: Command = Command::new();
    let mut to_skip: i8 = 0;

    result.command = CommandType::default();
    
    let mut i = 1;

    while i < args.len() {
        let arg = args[i].clone();

        if to_skip > 0 {
            to_skip -= 1;
            continue;
        }

        let check_ident = || -> Result<(), ArgParseError> {
            if result.command != CommandType::None {
                return Err(ArgParseError::IdentityPanic("Only one operation per run is allowed".to_string()))
            }

            Ok(())
        };

        match arg.as_str() {
            "build" => {
                if let Err(e) = check_ident() {
                    return Err(e);
                }

                result.command = CommandType::Build;

                if project.default_target == None && args.len() < i + 1 {
                    return Err(ArgParseError::ProjectPanic("No target specified".to_string()));
                }

                if project.default_target == None {
                    project.default_target = Some(args[i+1].clone());
                    to_skip += 1;
                }
            },
            "configure" => {
                if let Err(e) = check_ident() {
                    return Err(e);
                }

                result.command = CommandType::Configure;
            }
            "clean" => {
                if let Err(e) = check_ident() {
                    return Err(e)
                }

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