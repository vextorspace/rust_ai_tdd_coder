use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
use notify::Watcher;
use std::path::PathBuf;

pub struct WatchTcrCommand {
}

impl WatchTcrCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for WatchTcrCommand {
    fn execute(&self, assistant: Box<dyn Assistant>, path: PathBuf) -> anyhow::Result<Box<dyn Assistant>> {
        let path_clone = path.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            match res {
                Ok(event) => {
                    println!("EVENT: {event:?}");

                    match event.kind {
                        notify::EventKind::Modify(_) |
                        notify::EventKind::Create(_) |
                        notify::EventKind::Remove(_) => {
                            let result = assistant
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

    fn get_label(&self) -> &str {
        "watch_tcr"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watch_tcr_is_name() {
        let command: Box<dyn Command> = Box::new(WatchTcrCommand::new());

        assert_eq!(command.as_ref().get_label(), "watch_tcr");
    }
}