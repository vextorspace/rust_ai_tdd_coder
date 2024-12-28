use anyhow::{Result, anyhow};

pub struct CommandLineParser{

}

struct CommandLine {
    command: String,
    path: Option<String>
}

impl CommandLineParser {
    pub fn parse(args: &Vec<String>) -> Result<CommandLine> {
        Err(anyhow!("No command provided"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_command_returns_error() {
        let result = CommandLineParser::parse(&vec![]);
        assert!(result.is_err());
    }
}