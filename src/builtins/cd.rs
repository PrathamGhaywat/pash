use std::env;
use std::path::PathBuf;

use crate::CommandResult;

pub fn run(args: &[&str]) -> CommandResult {
    let target = args.get(0).copied().unwrap_or("~");

    let path = if target == "~" {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
    } else {
        PathBuf::from(target)
    };

    match env::set_current_dir(&path) {
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