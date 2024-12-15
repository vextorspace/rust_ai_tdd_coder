use crate::test_runner::test_results::TestResults;
use std::path::PathBuf;
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait TestProvider {
    fn run_tests(&self, path: PathBuf) -> TestResults;
}