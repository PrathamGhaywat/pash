/*
Command to create a new file
 */

use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::CommandResult;

pub fn run(args: &[&str]) -> CommandResult {
    // Get filename from arguments
    let target = match args.get(0) {
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