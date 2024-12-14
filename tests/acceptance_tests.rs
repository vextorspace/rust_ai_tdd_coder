use mockall::predicate::*;
use mockall::*;
use std::path::PathBuf;
use rust_ai_tdd_coder::test_runner::test_results::TestResults;

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
    
    // code is committed
}