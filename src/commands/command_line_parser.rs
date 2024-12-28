use anyhow::{anyhow, Result};
use std::path::PathBuf;
use super::command_line::CommandLine;

pub struct CommandLineParser{}

impl CommandLineParser {
    pub fn parse(args: &Vec<String>) -> Result<CommandLine> {
        Ok(
            CommandLine::new(
                Self::extract_command_or_err(args)?,
                Self::extract_path_or_none(args),
            )
        )
    }

    pub fn parse_command_line() -> Result<(String, PathBuf)> {
        let command_line = CommandLineParser::parse(&std::env::args().collect())?;

        let command = command_line.command;

        let path = match command_line.path {
            Some(p) => PathBuf::from(p),
            None => std::env::current_dir()?,
        };
        Ok((command, path))
    }

    fn extract_path_or_none(args: &Vec<String>) -> Option<String> {
        if args.len() > 2 {
            Some(args[2].clone())
        } else {
            None
        }
    }

    fn extract_command_or_err(args: &Vec<String>) -> Result<String> {
        if args.len() > 1 {
            Ok(args[1].clone())
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
        let result = CommandLineParser::parse(&vec!["program_name".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn command_is_first_argument() {
        let result = CommandLineParser::parse(&vec!["program_name".to_string(), "tcr".to_string()]);
        assert_eq!(result.unwrap().command, "tcr");
    }

    #[test]
    fn path_is_second_argument() {
        let result = CommandLineParser::parse(&vec!["program_name".to_string(), "tcr".to_string(), "path".to_string()]);
        assert_eq!(result.unwrap().path.unwrap(), "path");
    }
}

