use std::path::PathBuf;
use anyhow::Result;

pub trait Assistant {
    fn tcr(&self, path: PathBuf) -> Result<()>;
}