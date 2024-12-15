use crate::test_runner::test_provider::TestProvider;

pub struct Assistant {
    test_provider: Option<Box<dyn TestProvider>>,
}

impl Assistant {
    pub fn new() -> Assistant {
        Assistant {
            test_provider: None,
        }
    }

    pub fn with_test_provider(&mut self, test_provider: Box<dyn TestProvider>) -> &mut Self {
        self.test_provider = Some(test_provider);
        self
    }
}
#[cfg(test)]
mod tests {
    use mockall::{automock, mock};
    use super::*;
    use std::path::PathBuf;
    use crate::test_runner::test_results::TestResults;
    use crate::test_runner::test_provider::MockTestProvider;

    #[test]
    fn instatiates() {
        let mut mock_test_provider = MockTestProvider::new();
        mock_test_provider.expect_run_tests().times(0).returning(|_| TestResults::PASSED);

        let _ = Assistant::new().with_test_provider(Box::new(mock_test_provider));

    }
}