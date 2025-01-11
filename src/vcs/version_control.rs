use std::path::PathBuf;
use mockall::automock;
use anyhow::Result;
use downcast_rs::{impl_downcast, Downcast};
use crate::vcs::git_version_control::GitVersionControl;

#[cfg_attr(test, automock)]
pub trait VersionControl: Downcast + Send + Sync{
    fn commit(&self, path: &PathBuf, message: String) -> Result<()>;
    fn reject(&self, path: &PathBuf) -> Result<()>;
    fn diff(&self, path: &PathBuf) -> Result<String>;
    fn ignored(&self, path: &PathBuf) -> Result<bool>;
    fn boxed_clone(&self) -> Box<dyn VersionControl>;
}

impl_downcast!(VersionControl);

pub struct VersionControlBuilder ();

impl VersionControlBuilder {
    pub fn default() -> Box<dyn VersionControl> {
        Box::new(GitVersionControl::new())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn builder_default_makes_git() {
        let version_control = VersionControlBuilder::default();

        assert!(version_control.as_ref().downcast_ref::<GitVersionControl>().is_some());
    }
}