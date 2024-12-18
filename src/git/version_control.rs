use std::path::PathBuf;
use mockall::automock;
use anyhow::Result;

#[cfg_attr(test, automock)]
pub trait VersionControl {
    fn commit(&self, path: &PathBuf) -> Result<()>;
    fn reject(&self, path: &PathBuf) -> Result<()>;
}