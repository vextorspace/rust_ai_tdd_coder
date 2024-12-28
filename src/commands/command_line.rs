pub struct CommandLine {
    pub command: String,
    pub path: Option<String>
}

impl CommandLine {
    pub fn new(command: String, path: Option<String>) -> Self {
        CommandLine { command, path }
    }
}