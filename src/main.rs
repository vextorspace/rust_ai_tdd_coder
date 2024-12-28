use anyhow::{anyhow, Result};
use notify::Watcher;
use rust_ai_tdd_coder::{
    assistant::assistant_factory::AssistantFactory,
    commands::command_line_parser::CommandLineParser,
};

use std::path::PathBuf;

fn main() -> Result<()>{
    let command_line = CommandLineParser::parse(&std::env::args().collect())?;

    let command = std::env::args().nth(1).ok_or_else(|| anyhow!("No command argument provided"))?;
    let path = std::env::args()
        .nth(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    println!("Path: {:?}", path);

    match command.as_str() {
        "tcr" => AssistantFactory::default().tcr(path)?,
        "watch_tcr" => {
            watch_tcr(path.clone())?
        },
        _ => println!("Unknown command: {}", command),
    }

    Ok(())
}

fn watch_tcr(path: PathBuf) -> Result<()> {

    let path_clone = path.clone();

    let mut watcher = notify::recommended_watcher(move |res: std::result::Result<notify::Event, notify::Error>| {
        match res {
            Ok(event) => {
                println!("EVENT: {event:?}");

                match event.kind {
                    notify::EventKind::Modify(_) |
                    notify::EventKind::Create(_) |
                    notify::EventKind::Remove(_) => {
                        let result = AssistantFactory::default()
                            .tcr(path_clone.clone());
                        if let Err(e) = result {
                            eprintln!("Error running TCR: {e}");
                        }
                    },
                    _ => { },
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    })?;

    watcher.watch(&*path, notify::RecursiveMode::Recursive)?;

    println!("Press Ctrl-C to stop Watching for changes in: {:?}", path);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}