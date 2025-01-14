use super::version_control::VersionControl;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone)]
pub struct GitVersionControl{
    vcs_root: PathBuf,
}

impl GitVersionControl {
    pub fn new() -> Self {
        Self{
            vcs_root: PathBuf::from("."),
        }
    }
    
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            vcs_root: root,
        }
    }

    fn make_add_command(&self) -> Command {
        let mut command = Command::new("git");
        command.current_dir(self.vcs_root.clone());
        command.arg("add");
        command.arg(".");
        command
    }

    fn make_commit_command(&self, message: String) -> Command {
        let mut command = Command::new("git");
        command.current_dir(self.vcs_root.clone());
        command.arg("commit");
        command.arg("-m");
        command.arg(message);
        command
    }

    pub(crate) fn make_reject_command(&self) -> Command {
        let mut command = Command::new("git");
        command.current_dir(self.vcs_root.clone());
        command.arg("reset");
        command.arg("--hard");
        command.arg("HEAD");
        command
    }
}

impl VersionControl for GitVersionControl {
    fn commit(&self, message: String) -> Result<()>{
        let mut add_command = self.make_add_command();
        let add_status = add_command.status()?;
        if !add_status.success() {
            return Err(anyhow::anyhow!("Failed to add files"));
        }
        let mut command = self.make_commit_command(message);
        let commit_status = command.status()?;
        if !commit_status.success() {
            return Err(anyhow::anyhow!("Failed to commit files"));
        }
        Ok(())
    }

    fn reject(&self) -> Result<()> {
        println!("Rejecting changes");
        let mut command = self.make_reject_command();
        command.status()?;
        Ok(())
    }

    fn diff(&self) -> Result<String> {
        let output = Command::new("git")
            .current_dir(self.vcs_root.clone())
            .arg("diff")
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn ignored(&self, path: &PathBuf) -> Result<bool> {
       
        let output = Command::new("git")
            .current_dir(self.vcs_root.clone())
            .arg("check-ignore")
            .arg(path)
            .output()?;

        
        if let Some(file_name) = path.file_name() {
            if String::from_utf8_lossy(&output.stdout).contains(file_name.to_string_lossy().as_ref()) {
                return Ok(true);
            }
        }

        Ok(false)
    }
    
    fn boxed_clone(&self) -> Box<dyn VersionControl> {
        Box::new(self.clone())
    }
}

unsafe impl Send for GitVersionControl {}
unsafe impl Sync for GitVersionControl {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vcs::version_control::VersionControl;

    #[test]
    fn instantiate() {
        let _: Box<dyn VersionControl> = Box::new(GitVersionControl::new());
    }

    #[test]
    fn create_add_command() {
        let root = PathBuf::from("/home/");
        let provider = GitVersionControl::with_root(root.clone());
        let command = provider.make_add_command();

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
        assert_eq!(working_dir.unwrap(), root.as_path());
    }

    #[test]
    fn create_commit_command() {
        let root = PathBuf::from("/home/");
        let provider = GitVersionControl::with_root(root.clone());
        let commit_message = "commit message".to_string();

        let command = provider.make_commit_command(commit_message.clone());
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
        assert_eq!(message.unwrap().to_str().unwrap(), commit_message);

        let path = command.get_current_dir();
        assert!(path.is_some());
        assert_eq!(path.unwrap(), root.clone());
    }

    #[test]
    fn create_reject_command() {
        let root = PathBuf::from("/home/");
        let provider = GitVersionControl::with_root(root.clone());
        let command = provider.make_reject_command();
        let command_name = command.get_program();
        assert_eq!(command_name, "git");
        
        let mut args = command.get_args();
        let reset_argument = args.next();
        assert!(reset_argument.is_some());
        assert_eq!(reset_argument.unwrap(), "reset");

        let hard_argument = args.next();
        assert!(hard_argument.is_some());
        assert_eq!(hard_argument.unwrap(), "--hard");

        let head_argument = args.next();
        assert!(head_argument.is_some());
        assert_eq!(head_argument.unwrap(), "HEAD");

        let path = command.get_current_dir();
        assert!(path.is_some());
        assert_eq!(path.unwrap(), root.clone());
    }
    
    #[test]
    fn ignored_files_give_true() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitVersionControl::with_root(root);
        
        let path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");

        let result = provider.ignored(&path_buf);
        assert_eq!(result.unwrap(), true);
    }
    
    #[test]
    fn not_ignored_files_give_false() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitVersionControl::with_root(root);
        
        let path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

        let result = provider.ignored(&path_buf);
        assert_eq!(result.unwrap(), false);
    }
}
