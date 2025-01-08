use anyhow::Result;
use notify::Watcher;
use rust_ai_tdd_coder::assistant::assistant_factory::AssistantFactory;

use std::path::PathBuf;
use rust_ai_tdd_coder::assistant::assistant::Assistant;
use rust_ai_tdd_coder::commands::command_line_parser::CommandLineParser;
use rust_ai_tdd_coder::commands::commands::Commands;
use rust_ai_tdd_coder::commands::tcr_command::TcrCommand;
use rust_ai_tdd_coder::commands::watch_tcr_command::WatchTcrCommand;

fn main() -> Result<()>{
    let (command, path) = CommandLineParser::parse_command_line()?;

    let mut command_list = Commands::new();
    command_list.add(Box::new(TcrCommand::new()));
    command_list.add(Box::new(WatchTcrCommand::new()));

    command_list.execute(command.as_str(), Box::new(AssistantFactory::with_ai_commit()), path)?;

    Ok(())
}
