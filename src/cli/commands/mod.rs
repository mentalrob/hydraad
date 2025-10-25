
mod exit;
mod clear;
mod dc;
mod creds;
mod kerberos;

use std::future::Future;
use clap::{command, Parser, Subcommand};

use crate::{app::App, cli::commands::{clear::ClearArgs, creds::CredsArgs, dc::DcArgs, exit::ExitArgs, kerberos::KerberosArgs}};

macro_rules! handle_commands {
    ($command:expr, $app:expr, $($variant:ident),*) => {
        match $command {
            $(
                Commands::$variant(cmd) => cmd.execute($app).await,
            )*
        }
    };
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

impl Cli {
    pub async fn handle_command(&self, app: &mut App) -> Result<bool, String> {
        handle_commands!(
            &self.command, 
            app, 
            Exit,
            Clear,
            Dc,
            Creds,
            Kerberos
        )
    }
}

pub trait Command {
    fn execute(&self, app: &mut App) -> impl Future<Output = Result<bool, String>>;
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Exit the application")]
    Exit(ExitArgs),
    #[command(about = "Clear the screen")]
    Clear(ClearArgs),
    #[command(about = "Domain Controller Operations")]
    Dc(DcArgs),
    #[command(about = "Credential Management Operations")]
    Creds(CredsArgs),

    #[command(about = "Kerberos Operations")]
    Kerberos(KerberosArgs),
}