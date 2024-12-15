use crate::test_runner::test_provider::TestProvider;

pub struct Assistant {

}

impl Assistant {
    pub fn new() -> Assistant {
        Assistant {}
    }

    pub fn with_test_provider(&mut self, test_provider: Box<dyn TestProvider>) -> &mut Self {
        self
    }
}
#[cfg(test)]
mod tests {
    use mockall::{automock, mock};
    use super::*;
    use std::path::PathBuf;
    use crate::test_runner::test_results::TestResults;

    #[automock]
    pub trait TestProvider {
        fn run_tests(&self, path: PathBuf) -> TestResults;
    }

    #[test]
    fn instatiates() {
        let mut mock_test_provider = MockTestProvider::new();
        mock_test_provider.expect_run_tests().times(0);

        let _ = super::Assistant::new();

    }
}