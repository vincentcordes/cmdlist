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

fn check_field_for_query(
    command: &mut Command,
    query: &String,
    cmd: String,
    des: String,
    examples: Vec<Option<String>>,
    got: String,
) -> bool {
    let mut add_command = false;

    if command.command.is_some() {
        if cmd.contains(query) {
            add_command = true;
        }
    }
    if command.description.is_some() {
        if des.contains(query) {
            add_command = true;
        }
    }
    if examples.len() > 0 {
        for example in examples.iter() {
            if example.is_some() {
                if example.as_ref().unwrap().contains(query) {
                    add_command = true;
                }
            }
        }
    }

    if command.gotchas.is_some() {
        if got.contains(query) {
            add_command = true;
        }
    }

    return add_command;
}

// fn set_command_from_flags(
//     command: &mut Command,
//     args: &Args,
//     cmd: String,
//     des: String,
//     exa: String,
//     got: String,
// ) {
//     if args.command_off.is_none() || !args.command_off.unwrap() {
//         command.command = Some(String::from(cmd));
//     }
//     if args.descripton_off.is_none() || !args.descripton_off.unwrap() {
//         command.description = Some(String::from(des));
//     }
//     if args.example_off.is_none() || !args.example_off.unwrap() {
//         command.example = Some(String::from(exa));
//     }
//     if args.gotcha_off.is_none() || !args.gotcha_off.unwrap() {
//         command.gotchas = Some(String::from(got));
//     }
// }
