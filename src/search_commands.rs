use crate::command::Command;
use crate::arg_parse::Args;
use anyhow::*;

pub fn search_commands(args: &Args, commands: Vec<Command>) -> Result<Vec<Command>, Error> {
    let mut found_commands: Vec<Command> = Vec::new();

    let query_string = args.query.clone().unwrap();

    if args.query.is_some() {
        return get_all_fields(query_string, commands);
    }


    Ok(found_commands)
}

pub fn get_all_fields(query: String, commands: Vec<Command>) -> Result<Vec<Command>, Error> {
    let mut found_commands: Vec<Command> = Vec::new();
    for cmd in commands {

    }

    Ok(found_commands)
}
