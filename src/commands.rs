use std::process::Command;

use crate::builtins;
use crate::prompt;
use crate::result::CommandResult;

pub fn run(input: &str) -> bool {
    let mut parts = input.split_whitespace();
    let command = match parts.next() {
        Some(c) => c,
        None => return true,
    };

    let args: Vec<&str> = parts.collect();

    match command {
        //builtin commands then external
        "cd" => { //s witch to a certain directory
            print_result(builtins::cd::run(&args)); // c deez nuts
        }
        "pwd" => {
            println!("{}", prompt::current_dir()); //get current working dir
        }
        "touch" => {
            print_result(builtins::touch::run(&args));
        }
        "ls" => {
            print_result(builtins::ls::run(&args));
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

fn print_result(result: CommandResult) {
    let mode = std::env::var("PASH_MODE").unwrap_or_else(|_| "agent".to_string());

    // HUMAN MODE
    if mode == "human" {
        if !result.output.is_empty() {
            print!("{}", unescape(&result.output));
        }

        if let Some(err) = result.error {
            eprintln!("Error: {}", unescape(&err));
        }

        return;
    }

    // MACHINE MODE (structured JSON but readable output)
    let mut pretty = serde_json::to_value(&result).unwrap();

    // IMPORTANT: convert \n into real newlines for readability
    if let Some(obj) = pretty.as_object_mut() {
        if let Some(output) = obj.get_mut("output") {
            if let Some(s) = output.as_str() {
                *output = serde_json::Value::String(unescape(s));
            }
        }

        if let Some(error) = obj.get_mut("error") {
            if let Some(s) = error.as_str() {
                *error = serde_json::Value::String(unescape(s));
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&pretty).unwrap());
}

fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some(other) => {
                    // unknown escape → keep literally
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }

    result
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
