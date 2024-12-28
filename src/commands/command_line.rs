pub struct CommandLine {
    command: String,
    path: Option<String>
}

impl CommandLine {
    pub fn new(command: String, path: Option<String>) -> Self {
        CommandLine { command, path }
    }
}