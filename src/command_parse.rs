use crate::arg_parse::Args;
use crate::command::Command;
//use crate::utilities::*;
use anyhow::{Context, Error, Result};
//use serde_json::Value;

pub fn parse_commands(args: &Args) -> Result<Vec<Command>, Error> {
    let file_text =
        read_file(&args.path.clone().unwrap()).with_context(|| format!("Error reading file!"))?;
    return build_command_list(&args, &file_text);
}

pub fn read_file(path: &std::path::PathBuf) -> Result<String, Error> {
    let file_text = std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read file '{}'!", &path.display()))?;
    Ok(file_text)
}

pub fn build_command_list(args: &Args, file_text: &String) -> Result<Vec<Command>, Error> {
    let mut commands: Vec<Command> = serde_json::from_str(&file_text)?;

    // switches for query
    let mut use_query = false;
    let mut query = String::from("");
    if args.query.is_some() {
        use_query = true;
        query = args.query.clone().unwrap();
    }
    if use_query {
        commands.retain(|c| check_command_for_query(c, &query));
    }

    // Switch off what gets printed
    let commands = commands
        .iter()
        .map(|c| build_command_from_args(c, args))
        .collect();

    Ok(commands)
}

fn check_command_for_query(command: &Command, query: &String) -> bool {
    let mut command_contains = false;
    if command.command.is_some() {
        let cmd = command.command.clone().unwrap();
        if cmd.contains(query) {
            command_contains = true;
        }
    }

    if command.description.is_some() {
        let des = command.description.clone().unwrap();
        if des.contains(query) {
            command_contains = true;
        }
    }

    if command.examples.len() > 0 {
        for exa in command.examples.iter() {
            if exa.is_some() {
                let e = exa.clone().unwrap();
                if e.contains(query) {
                    command_contains = true;
                }
            }
        }
    }

    if command.gotchas.is_some() {
        let got = command.gotchas.clone().unwrap();
        if got.contains(query) {
            command_contains = true;
        }
    }

    command_contains
}

// sets what commands get printed from args
// print command function looks for None value
fn build_command_from_args(command: &Command, args: &Args) -> Command {
    let mut cmd = Command {
        command: command.command.clone(),
        description: command.description.clone(),
        examples: command.examples.clone(),
        gotchas: command.gotchas.clone(),
    };

    if args.command_off.is_some() && args.command_off.unwrap() {
        cmd.command = None;
    }
    if args.descripton_off.is_some() && args.descripton_off.unwrap() {
        cmd.description = None;
    }
    if args.example_off.is_some() && args.example_off.unwrap() {
        // this is enough here since print command function is looking for vec length > 0
        cmd.examples = vec![];
    }
    if args.gotcha_off.is_some() && args.gotcha_off.unwrap() {
        cmd.gotchas = None;
    }

    cmd
}
