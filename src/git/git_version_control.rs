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
        command.arg("add");
        command.arg(path);
        command
    }

    fn make_commit_command(&self, path: &PathBuf) -> Command {
        let mut command = Command::new("git");
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
    fn create_test_command() {
        let provider = GitVersionControl::new();
        let command = provider.make_commit_command(&PathBuf::from("/tests"));
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
    }
}
