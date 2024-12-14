use std::path::Path;
use anyhow::Result;

trait TestProvider {
    fn run_tests(&self, path: Path) -> Result<String>;
}