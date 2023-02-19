use anyhow::{Result, Error};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Clap Demo")]
#[command(author = "Vincent Cordes")]
#[command(version = "1.0.0")]
#[command(about = "A utility to print handy forgotten commands")]
#[command(long_about = None)]
pub struct Args{
    #[arg(short, long)]
    pub path: Option<std::path::PathBuf>,

    #[arg(short, long)]
    pub query: Option<String>,
}

impl Args {
    pub fn set_path(&mut self) {
        match &self.path {
            None => self.path = Some(std::path::PathBuf::from("data.json")),  // std::path::PathBuf::from("data.json"),
            Some(_) => (), // return std::path::PathBuf::from(path),
        }
    }
}

pub fn parse_args() -> Result<Args, Error> {
    let mut args = Args::parse();
    Args::set_path(&mut args);
    Ok(args)
}
