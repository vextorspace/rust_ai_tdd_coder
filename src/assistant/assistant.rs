use std::path::PathBuf;
use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub trait Assistant {
    fn tcr(&self, path: PathBuf) -> Result<()>;
}