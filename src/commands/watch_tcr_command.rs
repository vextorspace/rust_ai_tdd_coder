use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
use std::path::PathBuf;
pub(crate) struct WatchCommand {
}

impl WatchCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for WatchCommand {
    fn execute(&self, assistant: Box<&dyn Assistant>, path: PathBuf) -> anyhow::Result<()> {
        todo!()
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
        let command: Box<dyn Command> = Box::new(WatchCommand::new());

        assert_eq!(command.as_ref().get_label(), "watch_tcr");
    }

    #[tokio::test]
    async fn execute_does_not_call_assistant_on_no_change() {
        let mut assistant = MockAssistant::new();
        assistant.expect_tcr().times(0);
        let command = WatchCommand::new();



        use tokio::time::{sleep, Duration};
        use std::time::Instant;

        let handle = tokio::task::spawn(async move {
            command.execute(Box::new(&assistant), PathBuf::from("bob")).expect("should not fail");
        });


        tokio::task::spawn(async move {
            let start_time = Instant::now();
            sleep(Duration::from_secs(1)).await;

            handle.abort();
        });
    }

}