#[derive(Debug)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}