use std::fs;

use crate::result::CommandResult;

pub fn run(args: &[&str]) -> CommandResult {
    let target = args.get(0).copied().unwrap_or(".");

    let entries = match fs::read_dir(target) {
        Ok(entries) => entries,
        Err(e) => {
            return CommandResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            };
        }
    };

    let mut output = String::new();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                let name = match path.file_name() {
                    Some(name) => name.to_string_lossy(),
                    None => continue,
                };

                output.push_str(&format!("{}\n", name));
            }

            Err(e) => {
                return CommandResult {
                    success: false,
                    output,
                    error: Some(e.to_string()),
                };
            }
        }
    }

    CommandResult {
        success: true,
        output,
        error: None,
    }
}