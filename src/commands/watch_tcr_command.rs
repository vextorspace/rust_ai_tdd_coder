use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
use notify::{Event, Watcher};
use std::path::PathBuf;
use std::sync::Arc;
use crate::vcs::git_version_control::GitVersionControl;
use crate::vcs::version_control::VersionControl;
use crate::assistant::watch_lock::WatchLock;

pub struct WatchTcrCommand {
    vcs: Box<dyn VersionControl>,
}

impl WatchTcrCommand {
    pub(crate) fn is_good_event(vcs: Box<dyn VersionControl>, event: & Event, lock: &WatchLock) -> bool {
        let ignored = !event.paths.is_empty() && event.paths.iter().all(|path| {
            vcs.ignored(path).unwrap_or(false)
        });
            
        let kind_ok = match event.kind {
            notify::EventKind::Modify(notify::event::ModifyKind::Name(notify::event::RenameMode::Any)) => true,
            notify::EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Any)) => true,
            notify::EventKind::Create(notify::event::CreateKind::File) => true,
            notify::EventKind::Remove(notify::event::RemoveKind::Any) => true,
            _ => false,
        };

        let locked = lock.is_locked();

        if(kind_ok) {
            println!("Ignored: {ignored}, Kind: {kind_ok}, Locked: {locked}");
        }
        

        !ignored && kind_ok && !locked
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
        let vcs = Arc::new(self.vcs.boxed_clone());

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    if WatchTcrCommand::is_good_event(vcs.boxed_clone(), &event, assistant.get_lock()) {
                        println!("EVENT: {event:?}");
                        let result = assistant
                            .tcr(path_clone.clone());
                        if let Err(e) = result {
                            eprintln!("Error running TCR: {e}");
                        }
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

        let lock = WatchLock::new();
        let vcs = Box::new(GitVersionControl::new());
        assert!(WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }
    
    #[test]
    fn modify_file_data_is_good() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Any)),
            ..Default::default()
        };

        let lock = WatchLock::new();
        let vcs = Box::new(GitVersionControl::new());
        assert!(WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }
    
    #[test]
    fn modify_file_name_is_good() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Name(notify::event::RenameMode::Any)),
            ..Default::default()
        };

        let lock = WatchLock::new();
        let vcs = Box::new(GitVersionControl::new());
        assert!(WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }
    
    #[test]
    fn modify_file_metadata_is_bad() {
        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Metadata(notify::event::MetadataKind::Any)),
            ..Default::default()
        };

        let lock = WatchLock::new();
        let vcs = Box::new(GitVersionControl::new());

        assert!(!WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }
    
    #[test]
    fn delete_file_is_good() {
        let event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::Any),
            ..Default::default()
        };

        let lock = WatchLock::new();
        let vcs = Box::new(GitVersionControl::new());
        assert!(WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }
    
    #[test]
    fn modified_ignored_file_is_bad() {
        let path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");

        let event = Event {
            kind: EventKind::Modify(notify::event::ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![path_buf],
            ..Default::default()
        };

        let lock = WatchLock::new();
        let vcs = Box::new(GitVersionControl::new());
        assert!(!WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }

    #[test]
    fn new_file_is_bad_with_lock() {
        let event = Event::default()
            .set_kind(EventKind::Create(notify::event::CreateKind::File));
        let lock = WatchLock::new();
        lock.lock();
        let vcs = Box::new(GitVersionControl::new());
        assert!(!WatchTcrCommand::is_good_event(vcs, &event, &lock));
    }
 }