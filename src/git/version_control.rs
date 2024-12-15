use std::path::PathBuf;
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait VersionControl {
    fn commit(&self, path: PathBuf);
    fn reject(&self, path: PathBuf);
}