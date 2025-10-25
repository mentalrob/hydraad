use clap::Args;

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct ExitArgs;

impl Command for ExitArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        println!("Exiting...");
        Ok(true)
    }
}
