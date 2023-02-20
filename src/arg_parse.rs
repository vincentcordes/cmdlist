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

    #[arg(short, long)]
    pub command_off: Option<bool>,

    #[arg(short, long)]
    pub descripton_off: Option<bool>,

    #[arg(short, long)]
    pub example_off: Option<bool>,

    #[arg(short, long)]
    pub gotcha_off: Option<bool>,
}

impl Args {
    pub fn set_path(&mut self) {
        match &self.path {
            None => self.path = Some(std::path::PathBuf::from("data.json")),
            Some(_) => (),
        }
    }
}

pub fn parse_args() -> Result<Args, Error> {
    let mut args = Args::parse();
    Args::set_path(&mut args);
    Ok(args)
}
