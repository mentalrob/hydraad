mod shares;
mod list;

use clap::{command, Parser, Subcommand};

use crate::{app::App, cli::commands::{Command, smb::{list::ListArgs, shares::SharesArgs}}};

#[derive(Debug,Parser)]
pub struct SmbArgs {
    #[command(subcommand)]
    pub commands: SmbCommands
}

#[derive(Debug,Subcommand)]
pub enum SmbCommands {
    #[command(about = "List SMB shares")]
    Shares(SharesArgs),
    #[command(about = "List files in a share")]
    List(ListArgs)
}   

impl Command for SmbArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match &self.commands {
            SmbCommands::Shares(cmd) => cmd.execute(app).await,
            SmbCommands::List(cmd) => cmd.execute(app).await,
        }
    }
}
