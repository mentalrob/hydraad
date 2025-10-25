mod tgs;
mod tgt;
mod brute;

use clap::{Parser, Subcommand};

use crate::{app::App, cli::commands::{kerberos::{brute::BruteArgs, tgs::TgsArgs, tgt::TgtArgs}, Command}};

#[derive(Debug, Parser)]
pub struct KerberosArgs {
    #[command(subcommand)]
    pub commands: KerberosCommands
}

#[derive(Debug, Subcommand, Clone)]
pub enum KerberosCommands {
    Tgt(TgtArgs),
    Tgs(TgsArgs),
    Brute(BruteArgs),
}

impl Command for KerberosArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match &self.commands {
            KerberosCommands::Tgt(cmd) => cmd.execute(app).await,
            KerberosCommands::Tgs(cmd) => cmd.execute(app).await,
            KerberosCommands::Brute(cmd) => cmd.execute(app).await,
        }
    }
}
