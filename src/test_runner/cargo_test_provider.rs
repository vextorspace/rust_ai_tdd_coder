use super::test_provider::TestProvider;
use super::test_results::TestResults;
use std::path::PathBuf;

pub struct CargoTestProvider {}

impl CargoTestProvider {
    pub fn new() -> CargoTestProvider {
        CargoTestProvider {}
    }
}

impl TestProvider for CargoTestProvider {
    fn run_tests(&self, _path: &PathBuf) -> TestResults {
        TestResults::PASSED
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiates() {
        let _provider: Box<dyn TestProvider> = Box::new(CargoTestProvider::new());
    }


}

