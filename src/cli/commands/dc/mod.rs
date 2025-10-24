mod add;
mod list;
mod r#use;

use clap::{Parser, Subcommand};

use crate::{app::App, cli::commands::{dc::{add::AddArgs, list::ListArgs, r#use::UseArgs}, Command}};

#[derive(Debug, Parser)]
pub struct DcArgs {
    #[command(subcommand)]
    pub commands: DcCommands
}

#[derive(Debug, Subcommand)]
pub enum DcCommands {
    #[command(about = "Add a domain controller")]
    Add(AddArgs),
    #[command(about = "List all domain controllers")]
    List(ListArgs),
    #[command(about = "Use a domain controller")]
    Use(UseArgs),
}

impl Command for DcArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match &self.commands {
            DcCommands::Add(args) => args.execute(app).await,
            DcCommands::List(args) => args.execute(app).await,
            DcCommands::Use(args) => args.execute(app).await,
        }
    }
}