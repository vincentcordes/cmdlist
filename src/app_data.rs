use std::path::PathBuf;
use anyhow::{Error, Ok, Context};
use dirs::{ home_dir, config_dir };
use std::env;
use crate::utilities::print_type_of;

pub fn create_config_directory() -> Result<(), Error> {

    //if !config_dir().unwrap().is_dir()
    //{
    //    std::fs::create_dir(config_dir().unwrap())
    //        .with_context(|| format!("Unable to create .config directory!"))?;
    //}

    let a : String = String::from("linux");
    match env::consts::OS {
        a => println!("linux : OS MATCHED"),
    }

    println!("{}", env::consts::OS);

    //let cmdlist_dir = PathBuf::from(config_dir().unwrap());

    //if !cmdlist_dir.is_dir() {
    //    std::fs::create_dir(cmdlist_dir)
    //        .with_context(|| format!("Unable to create .config/cmdlist directory!"))?;
    //}


    println!("{} : {}",home_dir().unwrap().display(), config_dir().unwrap().display());

    Ok(())
}
