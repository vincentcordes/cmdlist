mod utilities;
mod command;
mod command_parse;
mod arg_parse;
//use utilities::*;
use crate::command::Command;
use crate::command_parse::*;
use crate::arg_parse::*;
use anyhow::{Context, Result, Error};


fn main() -> Result<(), Error> {
    let args = parse_args()
        .with_context(|| format!("Argument parse failed!"))?;

    let commands = parse_commands(&args)
        .with_context(|| format!("Failed to build command list!"))?;

    for cmd in commands {
        Command::print_command(cmd);
    }

    Ok(())
}
