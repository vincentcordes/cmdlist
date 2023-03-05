use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Command {
    pub command: Option<String>,
    pub description: Option<String>,
    pub examples: Vec<Option<String>>,
    pub gotchas: Option<String>,
}

impl Command {
    pub fn print_command(command: Command) {
        if command.command.is_some() {
            println!("    Command: {}", command.command.unwrap().bright_blue());
        }

        if command.description.is_some() {
            println!(
                "    Description: {}",
                command.description.unwrap().bright_magenta().italic()
            );
        }
        if command.examples.len() > 0 {
            println!("    Examples:");
        }
        for exa in command.examples.iter() {
            if exa.is_some() {
                println!(
                    "              {}",
                    exa.as_ref().unwrap().bright_green().bold()
                );
            }
        }
        if command.gotchas.is_some() {
            println!("    Gotchas: {}", command.gotchas.unwrap().bright_yellow());
        }
        println!();
    }
}
