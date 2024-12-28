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
                path: Self::extract_path_or_none(args),
            })
        } else {
            Err(anyhow!("No command provided"))
        }
    }

    fn extract_path_or_none(args: &Vec<String>) -> Option<String> {
        if args.len() > 1 {
            Some(args[1].clone())
        } else {
            None
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

    #[test]
    fn path_is_second_argument() {
        let result = CommandLineParser::parse(&vec!["tcr".to_string(), "path".to_string()]);
        assert_eq!(result.unwrap().path.unwrap(), "path");
    }
}