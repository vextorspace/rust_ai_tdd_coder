use std::path::PathBuf;
use dotenv::var;
use anyhow::{anyhow,Result};
use rust_ai_tdd_coder::ai::constant_commit_message::ConstantCommitMessage;
use rust_ai_tdd_coder::git::git_version_control::GitVersionControl;
use rust_ai_tdd_coder::test_runner::cargo_test_provider::CargoTestProvider;
use watchexec_signals::Signal;
use watchexec::Watchexec;
use rust_ai_tdd_coder::assistant::Assistant;

#[tokio::main]
async fn main() -> Result<()>{
    let command = std::env::args().nth(1).ok_or_else(|| anyhow!("No command argument provided"))?;
    let path = std::env::args()
        .nth(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    println!("Path: {:?}", path);

    match command.as_str() {
        "tcr" => make_assistant()?.tcr(path)?,
        "watch_tcr" => {
            watch_tcr(path.clone()).await?
        },
        _ => println!("Unknown command: {}", command),
    }

    Ok(())
}

fn make_assistant() -> Result<Assistant> {
    let test_provider = make_test_provider()?;

    let version_controller = make_version_control()?;

    let commit_generator = Box::new(ConstantCommitMessage::new("Working".to_string()));
    let assistant = rust_ai_tdd_coder::assistant::Assistant::new(test_provider, version_controller, commit_generator);
    Ok(assistant)
}

async fn watch_tcr(path: PathBuf) -> Result<()> {
    let wx = Watchexec::new(move |mut action| {
        for event in action.events.iter() {
            eprintln!("EVENT: {event:?}");
            let assistant = make_assistant();
            match assistant {
                Ok(assistant) => {
                    let result = assistant.tcr(path.clone());
                    if let Err(e) = result {
                        eprintln!("Error running TCR: {e}");
                    }
                },
                Err(e) => {
                    eprintln!("Error creating assistant: {e}");
                }
            }
        }

        // if Ctrl-C is received, quit
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit();
        }

        action
    })?;

    // watch the current directory
    wx.config.pathset(["."]);

    let _result = wx.main().await?;

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
