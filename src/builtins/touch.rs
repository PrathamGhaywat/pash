/*
Command to create a new file
 */

use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::result::CommandResult;

pub fn run(args: &[&str]) -> CommandResult {
    let force_run = args.contains(&"-f");
    // Get filename from arguments
    let target = match args.iter().find(|arg| !arg.starts_with('-')) {
        Some(name) => *name,
        None => {
            return CommandResult {
                success: false,
                output: String::new(),
                error: Some("missing filename".to_string()),
            };
        }
    };

    let path = PathBuf::from(target);

    // WHAT WILL HAPPEN IF THE FILE EXISTS? it will not continue. as easy as that
    if path.exists() && !force_run {
        return CommandResult { success: false, output: String::new(), error: Some(format!("{} already exists! Try running the command with -f flag to overwrite it and create a new empty file!", target)) };
    }

    match OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        {
            Ok(_) => CommandResult {
                success: true,
                output: String::new(),
                error: None,
            },
            Err(e) => CommandResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }
        }
}