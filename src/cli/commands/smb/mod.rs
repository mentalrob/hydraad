mod shares;
use clap::{command, Parser, Subcommand};

use crate::{app::App, cli::commands::{smb::shares::SharesArgs, Command}};

#[derive(Debug,Parser)]
pub struct SmbArgs {
    #[command(subcommand)]
    pub commands: SmbCommands
}

#[derive(Debug,Subcommand)]
pub enum SmbCommands {
    #[command(about = "List SMB shares")]
    Shares(SharesArgs)
}

impl Command for SmbArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match &self.commands {
            SmbCommands::Shares(cmd) => cmd.execute(app).await,
        }
    }
}
