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
        let mut command = Command::new("cargo");
        command.current_dir(path);
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

        let command_name = command.get_program();
        assert_eq!(command_name, "cargo");

        let mut args = command.get_args();
        let test_argument = args.next();
        assert!(test_argument.is_some());
        assert_eq!(test_argument.unwrap(), "test");

        let path = command.get_current_dir();
        assert!(path.is_some());
        assert_eq!(path.unwrap(), PathBuf::from("/tests").as_path());
    }


}

