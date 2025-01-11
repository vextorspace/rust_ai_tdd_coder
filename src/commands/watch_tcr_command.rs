use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
use notify::{Event, Watcher};
use std::path::PathBuf;
use crate::vcs::git_version_control::GitVersionControl;
use crate::vcs::version_control::VersionControl;

pub struct WatchTcrCommand {
    vcs: Box<dyn VersionControl>,
}

impl WatchTcrCommand {
    pub(crate) fn is_good_event(&self, event: &Event) -> bool {
        let ignored = !event.paths.is_empty() && event.paths.iter().all(|path| {
            self.vcs.ignored(path).unwrap_or(false)
        });
            
        let kind_ok = match event.kind {
            notify::EventKind::Modify(notify::event::ModifyKind::Name(notify::event::RenameMode::Any)) => true,
            notify::EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Any)) => true,
            notify::EventKind::Create(notify::event::CreateKind::File) => true,
            notify::EventKind::Remove(notify::event::RemoveKind::Any) => true,
            _ => false,
        };
        
        !ignored && kind_ok
    }
}

impl WatchTcrCommand {
    pub fn new() -> Self {
        Self {
            vcs: Box::new(GitVersionControl::new()),
        }
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
    use notify::{Event, EventKind};
    use super::*;

    #[test]
    fn watch_tcr_is_name() {
        let command: Box<dyn Command> = Box::new(WatchTcrCommand::new());

        assert_eq!(command.as_ref().get_label(), "watch_tcr");
    }
    
    #[test]
    fn new_event_is_good() {
        let event = Event::default()
            .set_kind(EventKind::Create(notify::event::CreateKind::File));

        let command = WatchTcrCommand::new();
        assert!(command.is_good_event(&event));
    }
    
    #[test]
    fn modify_file_data_is_good() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Any)),
            ..Default::default()
        };

        let command = WatchTcrCommand::new();
        assert!(command.is_good_event(&event));
    }
    
    #[test]
    fn modify_file_name_is_good() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Name(notify::event::RenameMode::Any)),
            ..Default::default()
        };

        let command = WatchTcrCommand::new();
        assert!(command.is_good_event(&event));
    }
    
    #[test]
    fn modify_file_metadata_is_bad() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Metadata(notify::event::MetadataKind::Any)),
            ..Default::default()
        };

        let command = WatchTcrCommand::new();
        assert!(!command.is_good_event(&event));
    }
    
    #[test]
    fn delete_file_is_good() {
        let event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::Any),
            ..Default::default()
        };

        let command = WatchTcrCommand::new();
        assert!(command.is_good_event(&event));
    }
    
    #[test]
    fn modified_ignored_file_is_bad() {
        let path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");

        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![path_buf],
            ..Default::default()
        };
        let command = WatchTcrCommand::new();
        assert!(!command.is_good_event(&event));
    }
 }