pub struct CommandLine {
    pub(crate) command: String,
    pub(crate) path: Option<String>
}

impl CommandLine {
    pub fn new(command: String, path: Option<String>) -> Self {
        CommandLine { command, path }
    }
}