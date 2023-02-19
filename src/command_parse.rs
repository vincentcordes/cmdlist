use crate::arg_parse::Args;
use crate::command::Command;
use anyhow::{ Context, Result, Error };
use serde_json::Value;

pub fn parse_commands(args: &Args) -> Result<Vec<Command>, Error> {
    let file_text = read_file(&args.path.clone().unwrap())
        .with_context(|| format!("Error reading file!"))?;
    return build_command_list(&args, &file_text);
}

pub fn read_file(path: &std::path::PathBuf) -> Result<String, Error> {
    let file_text =  std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read file '{}'!", &path.display()))?;
    Ok(file_text)
}

pub fn build_command_list(args: &Args, file_text: &String) -> Result<Vec<Command>, Error> {
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
        let des = &json_text["commands"][index]["description"].to_string();
        let exa = &json_text["commands"][index]["example"].to_string();
        let got = &json_text["commands"][index]["gotcha"].to_string();

        let mut command = Command{ ..Default::default() };
        // default add everything
        let mut add_command: bool = false;

        // build lines to add from args
        if args.command_off.is_none() || !args.command_off.unwrap() {
            command.command = Some(String::from(cmd));
        }
        if args.descripton_off.is_none() || !args.descripton_off.unwrap() {
            command.description = Some(String::from(des));
        }
        if args.example_off.is_none() || !args.example_off.unwrap() {
            command.example = Some(String::from(exa));
        }
        if args.gotcha_off.is_none() || !args.gotcha_off.unwrap() {
            command.gotchas = Some(String::from(got));
        }

        // if query and query is true
        if args.query.is_some() {
            if command.command.is_some() {
                if cmd.contains(&args.query.clone().unwrap()){
                    add_command = true;
                }
            }
            if command.description.is_some() {

                if des.contains(&args.query.clone().unwrap()){
                    add_command = true;
                }
            }
            if command.example.is_some() {

                if exa.contains(&args.query.clone().unwrap()){
                    add_command = true;
                }
            }
            if command.gotchas.is_some() {
                if got.contains(&args.query.clone().unwrap()){
                    add_command = true;
                }
            }
        }
        else {
            add_command = true;
        }

        if add_command {
            commands.push(command);
        }
    }
    Ok(commands)
}
