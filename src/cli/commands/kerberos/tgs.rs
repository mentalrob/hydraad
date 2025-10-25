use clap::Args;
use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args, Clone)]
pub struct TgsArgs {

}

impl Command for TgsArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        Ok(false)
    }
}
