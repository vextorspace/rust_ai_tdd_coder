use crate::test_runner::test_results::TestResults;
use std::path::PathBuf;

pub(crate) trait TestProvider {
    fn run_tests(&self, path: PathBuf) -> TestResults;
}