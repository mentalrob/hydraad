pub mod add;
pub mod list;
pub mod remove;

use clap::{Parser, Subcommand};

use crate::{app::App, cli::commands::{creds::{add::AddArgs, list::ListArgs, remove::RemoveArgs}, Command}};

#[derive(Debug, Parser)]
pub struct CredsArgs {
    #[command(subcommand)]
    pub commands: CredsCommands,
}

#[derive(Debug, Subcommand)]
pub enum CredsCommands {
    /// Add a new credential
    Add(AddArgs),
    /// List all credentials
    List(ListArgs),
    /// Remove a credential
    Remove(RemoveArgs),
}

impl Command for CredsArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match &self.commands {
            CredsCommands::Add(args) => args.execute(app).await,
            CredsCommands::List(args) => args.execute(app).await,
            CredsCommands::Remove(args) => args.execute(app).await,
        }
    }
}
