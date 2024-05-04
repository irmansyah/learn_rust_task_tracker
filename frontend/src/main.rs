mod apis;
mod commands;
mod selections;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <api_name>");
        return Ok(());
    }

    let command_name = &args[1];

    match command_name.as_str() {
        "task" => commands::command_task::execute(),
        "api" => commands::command_api_tool::execute(),
        _ => {
            println!("Invalid command.");
            Ok(())
        }
    }
}
