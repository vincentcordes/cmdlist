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
        build_file(&path)
            .with_context(|| format!("Unable to create default data file! {}", path.display()))?;
        println!("Default data file created at {}.", path.display());
    }
    Ok(path)
}

fn build_file(path: &PathBuf) -> Result<(), Error> {
    let data = r#"
    {
        "commands":[
            {
                "command": "ls",
                "description": "lists files",
                "example": "ls",
                "gotchas": "none - don't typo"
            },
            {
                "command": "exec",
                "description": "lists files",
                "example": "exec bash",
                "gotchas": "none - don't typo"
            }
        ]
    }
    "#;
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
