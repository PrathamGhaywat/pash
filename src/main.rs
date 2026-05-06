use std::io::{self, Write};
use std::process::Command;
use owo_colors::OwoColorize;
mod prompt;

fn main() {
    loop {
        print!("{:?} {}" , prompt::current_dir().bright_cyan(), "$ ".bright_green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("{}", "Failed to read input".on_red());
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match Command::new(command).args(&args).spawn() { // this will allow us to spawn new programs
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".bright_red(), e.bright_red());
            }
        }
    }
}
