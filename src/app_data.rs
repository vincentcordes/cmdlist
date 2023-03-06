use crate::Command;
use anyhow::{Context, Error, Ok};
use dirs::config_dir;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn application_defaults() -> Result<Option<PathBuf>, Error> {
    // Check for app config dir
    // Build if it doesnt exist
    let mut app_dir = build_app_directory()?;

    // Check for default data file
    // Build if it doesnt exist
    app_dir = build_default_data_file(&app_dir)?;

    Ok(Some(app_dir))
}

fn build_app_directory() -> Result<PathBuf, Error> {
    let mut app_dir = PathBuf::new();
    app_dir.push(PathBuf::from(config_dir().unwrap()));
    let dir_name = match env::consts::OS {
        "windows" => String::from("CmdList"),
        _ => String::from("cmdlist"),
    };
    app_dir.push(dir_name); // try to get a unique enough name
    if !app_dir.is_dir() {
        println!("Application settings directory does not exist. Attempting to build it.");
        std::fs::create_dir_all(&app_dir).with_context(|| {
            format!(
                "Unable to create application settings directory! {}",
                app_dir.display()
            )
        })?;
        println!(
            "Application settings directory created at {}.",
            app_dir.display()
        );
    }

    Ok(app_dir)
}

fn build_default_data_file(app_dir: &PathBuf) -> Result<PathBuf, Error> {
    let mut path = PathBuf::new();
    path.push(app_dir);
    path.push("data.json");

    if !path.is_file() {
        println!("Default data file does not exist. Attempting to build it.");
        let default_data = build_default_commmands()?;
        build_file(&default_data, &path)
            .with_context(|| format!("Unable to create default data file! {}", path.display()))?;
        println!("Default data file created at {}.", path.display());
    }
    Ok(path)
}

fn build_default_commmands() -> Result<Vec<Command>, Error> {
    let mut data: Vec<Command> = vec![];

    let mut examples1: Vec<Option<String>> = vec![];
    examples1.push(Some(String::from("ls")));
    examples1.push(Some(String::from("ls -l")));
    examples1.push(Some(String::from("ls -a")));
    let cmd1 = Command {
        command: Some(String::from("ls")),
        description: Some(String::from("Lists directory contents.")),
        examples: examples1,
        gotchas: Some(String::from("")),
    };

    let mut examples2: Vec<Option<String>> = vec![];
    examples2.push(Some(String::from("cd")));
    examples2.push(Some(String::from("cd ..")));
    examples2.push(Some(String::from("cd /")));
    let cmd2 = Command {
        command: Some(String::from("cd")),
        description: Some(String::from("Changes working directory.")),
        examples: examples2,
        gotchas: Some(String::from("")),
    };

    let mut examples3: Vec<Option<String>> = vec![];
    examples3.push(Some(String::from("cargo check")));
    examples3.push(Some(String::from("cargo build")));
    examples3.push(Some(String::from("cargo run")));
    let cmd3 = Command {
        command: Some(String::from("cargo")),
        description: Some(String::from("Rust package manager")),
        examples: examples3,
        gotchas: Some(String::from("")),
    };
    data.push(cmd1);
    data.push(cmd2);
    data.push(cmd3);
    Ok(data)
}

pub fn build_file(data: &Vec<Command>, path: &PathBuf) -> Result<(), Error> {
    let data_as_json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(path)?;
    file.write_all(data_as_json.as_bytes())?;
    Ok(())
}
