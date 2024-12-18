use super::test_provider::TestProvider;
use super::test_results::TestResults;
use std::path::PathBuf;
use std::process::Command;

pub struct CargoTestProvider {}

impl CargoTestProvider {
    pub fn new() -> CargoTestProvider {
        CargoTestProvider {}
    }

    fn make_test_command(&self, path: &PathBuf) -> Command {
        let mut command = Command::new(path);
        command.arg("test");
        command
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
    use std::process::CommandArgs;

    #[test]
    fn instantiates() {
        let _provider: Box<dyn TestProvider> = Box::new(CargoTestProvider::new());
    }

    #[test]
    fn creates_command() {
        let provider = CargoTestProvider::new();
        let command = provider.make_test_command(&PathBuf::from("/tests"));
        let mut args = command.get_args();
        let test_argument = args.next();
        assert!(test_argument.is_some());
    }


}

