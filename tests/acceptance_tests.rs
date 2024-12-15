use mockall::*;
use std::path::PathBuf;
use rust_ai_tdd_coder::test_runner::test_results::TestResults;
use anyhow::Result;
use rust_ai_tdd_coder::assistant::Assistant;

#[cfg(not(feature = "unit_tests"))]
#[test]
fn passing_test_commit_without_calling_ai() {
    use rust_ai_tdd_coder::test_runner::test_provider::MockTestProvider;
    use rust_ai_tdd_coder::git::version_control::MockVersionControl;
    use rust_ai_tdd_coder::ai::ai_coder::MockAiCoder;

    // given a test provider that returns PASSED
    let mut mock_test_provider = MockTestProvider::new();
    mock_test_provider
        .expect_run_tests()
        .times(1)
        .return_const(TestResults::PASSED);

    // expect that ai_coder.write_new_code is not called
    let mut mock_ai_coder = MockAiCoder::new();
    mock_ai_coder
        .expect_write_new_code()
        .times(0);

    // expect that version_control.commit is called
    // and that the reject is not called
    let mut mock_version_control = MockVersionControl::new();
    mock_version_control
        .expect_commit()
        .times(1);
    mock_version_control
        .expect_reject()
        .times(0);

    let assistant = Assistant::new()
        .with_test_provider(
            Box::new(mock_test_provider))
        .with_version_control(
            Box::new(mock_version_control))
        .with_ai_coder(
            Box::new(mock_ai_coder));

    assistant.run(PathBuf::from("path"));
}