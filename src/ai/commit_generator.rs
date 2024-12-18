use std::path::PathBuf;

#[cfg_attr(test, mockall::automock)]
pub trait CommitGenerator {
    fn generate_commit_message(&self, path: &PathBuf, diff: String) -> String;
}