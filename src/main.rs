mod add_command;
mod app_data;
mod arg_parse;
mod command;
mod command_parse;
mod utilities;
//use utilities::*;
use crate::add_command::*;
use crate::app_data::*;
use crate::arg_parse::*;
use crate::command::Command;
use crate::command_parse::*;
use anyhow::{Context, Error, Result};

fn main() -> Result<(), Error> {
    let mut args = parse_args().with_context(|| format!("Argument parse failed!"))?;

    let default_path = application_defaults()
        .with_context(|| format!("Unable to create default configuration!"))?;

    if args.path.is_none() {
        args.path = default_path;
    }

    let mut commands =
        parse_commands(&args).with_context(|| format!("Failed to build command list!"))?;

    if args.add.is_some() && args.add.unwrap() {
        add_command(&mut commands, &args.path.unwrap())?;
    } else {
        for cmd in commands {
            Command::print_command(cmd);
        }
    }

    Ok(())
}
