use anyhow::{Result, anyhow};

pub struct CommandLineParser{

}

pub struct CommandLine {
    command: String,
    path: Option<String>
}

impl CommandLineParser {
    pub fn parse(args: &Vec<String>) -> Result<CommandLine> {
        Ok(CommandLine {
            command: Self::extract_command_or_err(args)?,
            path: Self::extract_path_or_none(args),
        })
    }

    fn extract_path_or_none(args: &Vec<String>) -> Option<String> {
        if args.len() > 1 {
            Some(args[1].clone())
        } else {
            None
        }
    }

    fn extract_command_or_err(args: &Vec<String>) -> Result<String> {
        if args.len() > 0 {
            Ok(args[0].clone())
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

    #[test]
    fn path_is_second_argument() {
        let result = CommandLineParser::parse(&vec!["tcr".to_string(), "path".to_string()]);
        assert_eq!(result.unwrap().path.unwrap(), "path");
    }
}