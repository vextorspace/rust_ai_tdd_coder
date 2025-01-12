use std::path::PathBuf;
use anyhow::Result;
use crate::assistant::watch_lock::WatchLock;

#[cfg_attr(test, mockall::automock)]
pub trait Assistant: Send {
    fn tcr(&self, path: PathBuf) -> Result<()>;
    fn get_lock(&self) -> &WatchLock;
}