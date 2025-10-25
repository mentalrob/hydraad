use clap::Args;
use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args, Clone)]
pub struct BruteArgs {

}

impl Command for BruteArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        Ok(false)
    }
}