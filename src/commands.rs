use std::process::Command;

use crate::builtins;
use crate::prompt;

pub fn run(input: &str) -> bool {
    let mut parts = input.split_whitespace();
    let command = match parts.next() {
        Some(c) => c,
        None => return true,
    };

    let args: Vec<&str> = parts.collect();

    match command {
        //builtin commands then external
        "cd" => {
            builtins::cd::run(&args); // c deez nuts
        }
        "pwd" => {
            println!("{}", prompt::current_dir());
        }
        "exit"  => {
            return false;
        }

        _  => {
            run_external(command, &args);
        }
    }

    true
}

fn run_external(cmd: &str, args: &[&str]) {
    match Command::new(cmd).args(args).spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}