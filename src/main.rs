mod utilities;
mod command;
mod command_parse;
mod arg_parse;
mod app_data;
//use utilities::*;
use crate::command::Command;
use crate::command_parse::*;
use crate::arg_parse::*;
use crate::app_data::*;
use anyhow::{Context, Result, Error};


fn main() -> Result<(), Error> {
    let args = parse_args()
        .with_context(|| format!("Argument parse failed!"))?;

    create_config_directory()
        .with_context(|| format!("Unable to create default configuration"))?;

    let commands = parse_commands(&args)
        .with_context(|| format!("Failed to build command list!"))?;

    for cmd in commands {
        Command::print_command(cmd);
    }

    Ok(())
}
