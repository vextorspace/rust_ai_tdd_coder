use crate::test_runner::test_results::TestResults;
use std::path::PathBuf;
use dotenv::var;
use downcast_rs::{impl_downcast, Downcast};
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait TestProvider: Downcast {
    fn run_tests(&self, path: &PathBuf) -> TestResults;
}

impl_downcast!(TestProvider);

pub struct TestProviderFactory {}

impl TestProviderFactory {
    pub(crate) fn default() -> Box<dyn TestProvider> {
        let lang_result = var("TARGET_LANGUAGE");
        let l = if let Ok(lang) = lang_result {
            lang.to_string()
        } else {
            "rust".to_string()
        };

        TestProviderFactory::from_lang(l.as_str())
    }

    pub fn from_lang(lang: &str) -> Box<dyn TestProvider> {
        match lang {
            "rust" => Box::new(crate::test_runner::cargo_test_provider::CargoTestProvider::new()),
            _ => panic!("Language not supported")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_runner::cargo_test_provider::CargoTestProvider;
    use super::*;

    #[test]
    fn from_lang_makes_cargo_for_rust() {
        let provider = TestProviderFactory::from_lang("rust");

        assert!(provider.is::<CargoTestProvider>());
    }

    #[test]
    fn default_looks_for_env_lang() {
        let provider = TestProviderFactory::default();
        assert!(provider.is::<CargoTestProvider>());
    }
}