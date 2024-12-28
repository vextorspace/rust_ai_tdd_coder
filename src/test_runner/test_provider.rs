use crate::test_runner::test_results::TestResults;
use std::path::PathBuf;
use downcast_rs::{impl_downcast, Downcast};
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait TestProvider: Downcast {
    fn run_tests(&self, path: &PathBuf) -> TestResults;
}

impl_downcast!(TestProvider);

pub struct TestProviderFactory {}

impl TestProviderFactory {
    pub fn from_lang(lang: &str) -> Box<dyn TestProvider> {
        match lang {
            "rust" => Box::new(crate::test_runner::cargo_test_provider::CargoTestProvider::new()),
            _ => panic!("Language not supported")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_lang_makes_cargo_for_rust() {
        let provider = TestProviderFactory::from_lang("rust");

        assert!(provider.is::<crate::test_runner::cargo_test_provider::CargoTestProvider>());
    }
}