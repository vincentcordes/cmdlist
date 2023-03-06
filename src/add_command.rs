use crate::{app_data::build_file, command::Command};
use anyhow::{Error, Result};
use std::{io, path::PathBuf};

pub fn add_command(commands: &mut Vec<Command>, path: &PathBuf) -> Result<(), Error> {
    let new_command = build_new_command()?;
    commands.push(new_command);
    build_file(commands, path)?;
    Ok(())
}

fn build_new_command() -> Result<Command, Error> {
    let mut command = Command {
        ..Default::default()
    };

    // command
    let mut input_str = String::new();
    println!("Enter command: ");
    io::stdin().read_line(&mut input_str)?;
    let trim_str = input_str.trim();
    if input_str != "" {
        command.command = Some(String::from(trim_str));
    }

    // description
    let mut input_str = String::new();
    println!("Enter description: ");
    io::stdin().read_line(&mut input_str)?;
    let trim_str = input_str.trim();
    if input_str != "" {
        command.description = Some(String::from(trim_str));
    }
    // examples
    command.examples = vec![];
    let mut loop_flag: usize = 3;
    while loop_flag > 2 {
        let mut input_str = String::new();
        println!("Enter example (nothing to quit): ");
        loop_flag = io::stdin().read_line(&mut input_str)?;

        let trim_str = input_str.trim();

        if loop_flag > 2 {
            command.examples.push(Some(String::from(trim_str)));
        }
    }

    // gotchas
    let mut input_str = String::new();
    println!("Enter gotchas: ");
    io::stdin().read_line(&mut input_str)?;
    let trim_str = input_str.trim();
    if input_str != "" {
        command.gotchas = Some(String::from(trim_str));
    }

    Ok(command)
}
