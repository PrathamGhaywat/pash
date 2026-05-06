use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env};
use owo_colors::OwoColorize;

fn normalize(path: &Path) -> String {
    // normalizing path: because shitty windows gives us \\ instead of beautiful Unix style /
    path.to_string_lossy().replace("\\", "/")
}

fn current_dir() -> String {
    /* 
    This will get the current dir. The need for this is because the crate returns the full file path, 
    but we want the relative file path from the user bin.
    */
    let path = env::current_dir().unwrap_or_else(|_| PathBuf::from("?"));
    let s = normalize(&path);

    if let Some(home) = dirs::home_dir() {
        let home = normalize(&home);

        if let Some(stripped) = s.strip_prefix(&home) {
            return format!("~{}", stripped);
        }
    }

    return s;
}

fn main() {
    loop {
        print!("{:?} {}" , current_dir().bright_cyan(), "$ ".bright_green());
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

        match Command::new(command).args(&args).spawn() {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => {
                eprintln!("{} {}", "Error:".bright_red(), e.bright_red());
            }
        }
    }
}
