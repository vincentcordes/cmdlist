use anyhow::{Context, Result, Error};
use serde_json::Value;
use colored::*;

pub struct Command {
     pub command: String,
     pub description: String,
     pub example: String,
     pub gotchas: String,
}

impl Command {
    pub fn print_command(command: &Command) {
        println!("    Command: {}", command.command.bright_blue());
        println!("    Description: {}", command.description.bright_magenta().italic());
        println!("    Example: {}", command.example.bright_green().bold());
        println!("    Gotchas: {}", command.gotchas.bright_yellow());
        println!();
    }
}

pub fn parse_commands(path: &std::path::PathBuf) -> Result<Vec<Command>, Error> {
    let file_text = read_file(&path)
        .with_context(|| format!("Error reading file!"))?;
    return build_command_list(&file_text);
}

pub fn read_file(path: &std::path::PathBuf) -> Result<String, Error> {
    let file_text =  std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read file '{}'!", &path.display()))?;
    Ok(file_text)
}

pub fn build_command_list(file_text: &String) -> Result<Vec<Command>, Error> {
   let json_text = serde_json::from_str::<Value>(&file_text)
       .with_context(|| format!("Invalid json format!"))
       .unwrap();

    let elements_count = json_text["commands"].as_array()
        .with_context(|| format!("Unable to find 'commands' section in data file!"))
        .unwrap().
        len();

    let mut commands : Vec<Command> = vec![];

    for index in 0..elements_count {
        let cmd = &json_text["commands"][index]["command"].to_string();
        let des = &json_text["commands"][index]["command"].to_string();
        let exa = &json_text["commands"][index]["command"].to_string();
        let got = &json_text["commands"][index]["command"].to_string();
        let command = Command {
            command: String::from(cmd),
            description: String::from(des),
            example: String::from(exa),
            gotchas: String::from(got),
        };

       commands.push(command);
    }
    Ok(commands)
}
