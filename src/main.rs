mod utilities;
mod command;
mod command_parse;
mod arg_parse;
mod search_commands;
//use utilities::*;
use crate::command::Command;
use crate::command_parse::*;
use crate::arg_parse::*;
//use crate::search_commands::*;
use anyhow::{Context, Result};


fn main() -> Result<()> {
    let args = parse_args()
        .with_context(|| format!("Argument parse failed!"))?;

    // load from file
    let commands = parse_commands(&args)
        .with_context(|| format!("Faild to build command list!"))
        .unwrap();

    // Skip search if no qurery is provided
    //if args.query.is_some() {
    //    let commands = search_commands(&args, commands)
    //        .with_context(|| format!("Faild to build command list!"))
    //        .unwrap();
    //}

    for cmd in commands {
        Command::print_command(cmd);
    }

    Ok(())
}
