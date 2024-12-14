use mockall::*;
use std::path::PathBuf;
use rust_ai_tdd_coder::test_runner::test_results::TestResults;
use anyhow::Result;

#[cfg(not(feature = "unit_tests"))]
#[test]
fn passing_test() {
    // given a set of passing tests
    mock! {
        pub TestProvider {
            fn run_test(&self, path: PathBuf) -> TestResults;
        }
    }

    let mut mock_provider = MockTestProvider::new();

    // Expect `run_test` to be called exactly once and return `TestResults::PASSED`
    mock_provider
        .expect_run_test()
        .times(1)
        .return_const(TestResults::PASSED);

    // ai is not queried


    mock! {
    pub AiCoder {
        fn write_new_code(&self, code: String, tests: String) -> Result<String>;
    }
}

    let mut mock_ai_coder = MockAiCoder::new();

    // Expect `write_new_code` to never be called
    mock_ai_coder
        .expect_write_new_code()
        .times(0);

    // code is committed


}