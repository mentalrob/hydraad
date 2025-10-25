use std::path::PathBuf;
use clap::Args;

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct SaveFileArgs {
    /// Path to save the credentials file
    pub path: PathBuf,
}

impl Command for SaveFileArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match app.credential_storage().save_to_file(&self.path) {
            Ok(_) => {
                println!("Credentials saved successfully to: {}", self.path.display());
                Ok(false)
            }
            Err(e) => Err(format!("Failed to save credentials to file: {}", e))
        }
    }
}