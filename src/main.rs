use dotenv::var;
use anyhow::{anyhow,Result};
use rust_ai_tdd_coder::git::git_version_control::GitVersionControl;
use rust_ai_tdd_coder::test_runner::cargo_test_provider::CargoTestProvider;

fn main() -> Result<()>{
    let lang = var("TARGET_LANGUAGE")?.to_lowercase();
    let test_provider = match lang.as_str() {
        "rust" => {
            Ok(Box::new(CargoTestProvider::new()))
        },
        _ => {
            Err(anyhow!("Unsupported language: {}", lang))
        }
    }?;

    let vcs = var("VERSION_CONTROL")?.to_lowercase();
    let version_controller = match vcs.as_str() {
        "git" => {
            Ok(Box::new(GitVersionControl::new()))
        },
        _ => {
            Err(anyhow!("Unsupported version control system: {}", vcs))
        }
    }?;

    // let assistant = rust_ai_tdd_coder::assistant::Assistant::new(test_provider, version_controller, );

    Ok(())
}
