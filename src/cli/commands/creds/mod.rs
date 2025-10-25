mod add;
mod list;
mod remove;
mod r#use;
mod save_file;
mod load_file;

use clap::{Parser, Subcommand};

use crate::{app::App, cli::commands::{creds::{add::AddArgs, list::ListArgs, remove::RemoveArgs, r#use::UseArgs, save_file::SaveFileArgs, load_file::LoadFileArgs}, Command}};

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
    /// Use a credential
    Use(UseArgs),

    /// Save credentials to file
    SaveFile(SaveFileArgs),
    /// Load credentials from file
    LoadFile(LoadFileArgs)
}

impl Command for CredsArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match &self.commands {
            CredsCommands::Add(args) => args.execute(app).await,
            CredsCommands::List(args) => args.execute(app).await,
            CredsCommands::Remove(args) => args.execute(app).await,
            CredsCommands::Use(args) => args.execute(app).await,
            CredsCommands::SaveFile(args) => args.execute(app).await,
            CredsCommands::LoadFile(args) => args.execute(app).await,
        }
    }
}
