use super::version_control::VersionControl;
use std::path::PathBuf;
use std::process::Command;
use anyhow::Result;

pub struct GitVersionControl{

}

impl GitVersionControl {
    pub(crate) fn new() -> Self {
        Self{}
    }

    fn make_add_command(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("git");
        command.current_dir(path);
        command.arg("add");
        command.arg(".");
        command
    }

    fn make_commit_command(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("git");
        command.current_dir(path);
        command.arg("commit");
        command.arg("-m");
        command.arg("working");
        command
    }
}

impl VersionControl for GitVersionControl {
    fn commit(&self, path: &PathBuf) -> Result<()>{
        let mut add_command = self.make_add_command(path);
        add_command.status()?;
        let mut command = self.make_commit_command(path);
        command.status()?;
        Ok(())
    }

    fn reject(&self, _path: &PathBuf) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::git::version_control::VersionControl;
    use super::*;

    #[test]
    fn instantiate() {
        let _: Box<dyn VersionControl> = Box::new(GitVersionControl::new());
    }

    #[test]
    fn create_add_command() {
        let provider = GitVersionControl::new();
        let path_buf = PathBuf::from("/tests".clone());
        let command = provider.make_add_command(&path_buf);
        let command_name = command.get_program();
        assert_eq!(command_name, "git");
        let mut args = command.get_args();
        let add_argument = args.next();
        assert!(add_argument.is_some());
        assert_eq!(add_argument.unwrap(), "add");

        let path = args.next();
        assert!(path.is_some());
        assert_eq!(path.unwrap(), ".");

        let working_dir = command.get_current_dir();
        assert!(working_dir.is_some());
        assert_eq!(working_dir.unwrap(), path_buf.as_path());
    }

    #[test]
    fn create_test_command() {
        let provider = GitVersionControl::new();
        let path_buf = PathBuf::from("/tests");
        let command = provider.make_commit_command(&path_buf);
        let command_name = command.get_program();
        assert_eq!(command_name, "git");
        let mut args = command.get_args();
        let commit_argument = args.next();
        assert!(commit_argument.is_some());
        assert_eq!(commit_argument.unwrap(), "commit");

        let message_argument = args.next();
        assert!(message_argument.is_some());
        assert_eq!(message_argument.unwrap(), "-m");

        let message = args.next();
        assert!(message.is_some());
        assert!(!message.unwrap().is_empty());

        let path = command.get_current_dir();
        assert!(path.is_some());
        assert_eq!(path.unwrap(), path_buf.as_path());
    }
}
