use std::path::PathBuf;

pub trait VersionControl {
    fn commit(&self, path: PathBuf);
    fn reject(&self, path: PathBuf);
}