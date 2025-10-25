use std::path::PathBuf;
use clap::Args;

use crate::{app::App, cli::commands::Command, stores::credentials_store::CredentialsStore};

#[derive(Debug, Args)]
pub struct LoadFileArgs {
    /// Path to load the credentials file from
    pub path: PathBuf,
}

impl Command for LoadFileArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        match CredentialsStore::load_from_file(&self.path) {
            Ok(loaded_store) => {
                let loaded_count = loaded_store.len();
                *app.credential_storage() = loaded_store;
                println!("Credentials loaded successfully from: {}", self.path.display());
                println!("Loaded {} credentials", loaded_count);
                Ok(false)
            }
            Err(e) => Err(format!("Failed to load credentials from file: {}", e))
        }
    }
}
