use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
use std::path::PathBuf;
use crate::assistant::assistant_factory::AssistantFactory;
use notify::Watcher;

pub(crate) struct WatchTcrCommand {
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
                            let result = AssistantFactory::with_ai_commit()
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

        Ok(assistant)
    }

    fn get_label(&self) -> &str {
        "watch_tcr"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assistant::assistant::MockAssistant;

    #[test]
    fn watch_tcr_is_name() {
        let command: Box<dyn Command> = Box::new(WatchTcrCommand::new());

        assert_eq!(command.as_ref().get_label(), "watch_tcr");
    }
}