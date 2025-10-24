use crate::{app::App, cli::commands::Command, utils::cli_utils::clear_screen};
use clap::Args;

#[derive(Debug, Args)]
pub struct ClearArgs;

impl Command for ClearArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        clear_screen();
        Ok(false)
    }
}
