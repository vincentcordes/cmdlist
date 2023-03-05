use anyhow::{Error, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Command List")]
#[command(author = "Vincent Cordes")]
#[command(version = "1.0.0")]
#[command(about = "A utility to print handy forgotten commands")]
#[command(long_about = None)]
pub struct Args {
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

pub fn parse_args() -> Result<Args, Error> {
    let args = Args::parse();
    Ok(args)
}
