mod prompt;
mod commands;
mod builtins;
mod result;

use std::io::{Write};
use owo_colors::OwoColorize;
use result::CommandResult;
fn main() {
    loop {
        print!("{} {} ", prompt::current_dir().bright_cyan(), "$".bright_green());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let should_continue = commands::run(input);

        if !should_continue {
            break;
        }
    }
}
