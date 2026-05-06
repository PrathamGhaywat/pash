use std::path::{Path, PathBuf};
use std::env;

fn normalize(path: &Path) -> String {
    // normalizing path: because shitty windows gives us \\ instead of beautiful Unix style /
    path.to_string_lossy().replace("\\", "/")
}

pub fn current_dir() -> String {
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