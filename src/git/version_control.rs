use std::path::PathBuf;
use mockall::automock;
use anyhow::Result;

#[cfg_attr(test, automock)]
pub trait VersionControl {
    fn commit(&self, path: &PathBuf, message: String) -> Result<()>;
    fn reject(&self, path: &PathBuf) -> Result<()>;
    fn diff(&self, path: &PathBuf) -> Result<String>;
}