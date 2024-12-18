use std::error::Error;
use dotenv::var;
use rust_ai_tdd_coder::assistant::Assistant;
use rust_ai_tdd_coder::test_runner::cargo_test_provider::CargoTestProvider;

fn main() -> Result<(), Box<dyn Error>>{
    let lang = var("TARGET_LANGUAGE")?;
    match lang.as_str() {
        "rust" => {
            let test_provider = Box::new(CargoTestProvider::new());

        },
        _ => {
            eprintln!("Unsupported language: {}", lang);
        }
    }

    Ok(())
}
