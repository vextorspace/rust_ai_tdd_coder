use std::path::PathBuf;
use std::any::Any;
use mockall::automock;
use anyhow::Result;
use downcast_rs::{impl_downcast, Downcast};
use crate::git::git_version_control::GitVersionControl;

#[cfg_attr(test, automock)]
pub trait VersionControl: Downcast {
    fn commit(&self, path: &PathBuf, message: String) -> Result<()>;
    fn reject(&self, path: &PathBuf) -> Result<()>;
    fn diff(&self, path: &PathBuf) -> Result<String>;
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