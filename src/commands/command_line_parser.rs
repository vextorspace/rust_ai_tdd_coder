use anyhow::{Result, anyhow};

pub struct CommandLineParser{

}

struct CommandLine {
    command: String,
    path: Option<String>
}

impl CommandLineParser {
    pub fn parse(args: &Vec<String>) -> Result<CommandLine> {
        if args.len() > 0 {
            Ok(CommandLine {
                command: args[0].clone(),
                path: None,
            })
        } else {
            Err(anyhow!("No command provided"))
        }
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

    #[test]
    fn command_is_first_argument() {
        let result = CommandLineParser::parse(&vec!["tcr".to_string()]);
        assert_eq!(result.unwrap().command, "tcr");
    }
}