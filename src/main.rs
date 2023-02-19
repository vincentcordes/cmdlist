mod command_parse;
mod arg_parse;
use crate::command_parse::*;
use crate::arg_parse::*;
use anyhow::{Context, Result};

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

fn main() -> Result<()> {
    let args = parse_args()
        .with_context(|| format!("Argument parse failed!"))?;

    let commands = parse_commands(&args.path.unwrap())
        //.with_context(|| format!("Faild to build command list!"))
        .unwrap();

    for cmd in commands {
        Command::print_command(&cmd);
    }

    Ok(())
}
