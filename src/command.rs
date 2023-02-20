use colored::Colorize;

#[derive(Default)]
pub struct Command {
     pub command: Option<String>,
     pub description: Option<String>,
     pub example: Option<String>,
     pub gotchas: Option<String>,
}

impl Command {
    pub fn print_command(command: Command) {
        if command.command.is_some(){
            println!("    Command: {}",
                     command.command.unwrap().bright_blue());
        }

        if command.description.is_some(){
            println!("    Description: {}",
                     command.description.unwrap().bright_magenta().italic());
        }
        if command.example.is_some() {

            println!("    Example: {}", command.example.unwrap().bright_green().bold());
        }
        if command.gotchas.is_some() {
            println!("    Gotchas: {}", command.gotchas.unwrap().bright_yellow());
        }
        println!();
    }
}


