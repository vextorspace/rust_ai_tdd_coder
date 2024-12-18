use dotenv::var;
use anyhow::{anyhow,Result};
use rust_ai_tdd_coder::ai::constant_commit_message::ConstantCommitMessage;
use rust_ai_tdd_coder::git::git_version_control::GitVersionControl;
use rust_ai_tdd_coder::test_runner::cargo_test_provider::CargoTestProvider;

fn main() -> Result<()>{
    let test_provider = make_test_provider()?;

    let version_controller = make_version_control()?;

    let commit_generator = Box::new(ConstantCommitMessage::new("Working".to_string()));
    let assistant = rust_ai_tdd_coder::assistant::Assistant::new(test_provider, version_controller,commit_generator);

    Ok(())
}

fn make_version_control() -> Result<Box<GitVersionControl>> {
    let vcs = var("VERSION_CONTROL")?.to_lowercase();
    let version_controller = match vcs.as_str() {
        "git" => {
            Ok(Box::new(GitVersionControl::new()))
        },
        _ => {
            Err(anyhow!("Unsupported version control system: {}", vcs))
        }
    }?;
    Ok(version_controller)
}

fn make_test_provider() -> Result<Box<CargoTestProvider>> {
    let lang = var("TARGET_LANGUAGE")?.to_lowercase();
    let test_provider = match lang.as_str() {
        "rust" => {
            Ok(Box::new(CargoTestProvider::new()))
        },
        _ => {
            Err(anyhow!("Unsupported language: {}", lang))
        }
    }?;
    Ok(test_provider)
}
