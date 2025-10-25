use clap::Args;
use log::info;
use winston::log;

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Credential ID to remove (can be partial, will match the beginning)
    pub credential_id: String,
    
    /// Force removal without confirmation
    #[arg(short, long)]
    pub force: bool,
}

impl Command for RemoveArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        // Find credentials that match the provided ID (partial match)
        let all_credentials = app.credential_storage().get_all_credentials();
        let matching_credentials: Vec<_> = all_credentials
            .iter()
            .filter(|cred| cred.id.starts_with(&self.credential_id))
            .collect();

        match matching_credentials.len() {
            0 => {
                return Err(format!("No credential found with ID starting with '{}'", self.credential_id));
            }
            1 => {
                let credential = matching_credentials[0];
                let full_id = credential.id.clone();
                
                if !self.force {
                    println!("Are you sure you want to remove credential:");
                    println!("  ID: {}", credential.id);
                    println!("  Username: {}", credential.username);
                    println!("  Type: {:?}", credential.credential_type);
                    println!("  Source: {}", credential.source);
                    println!();
                    println!("This action cannot be undone. Use --force to skip this confirmation.");
                    return Ok(false);
                }
                
                // Remove the credential
                match app.credential_storage().remove_credential(&full_id) {
                    Some(_) => {
                        log!(info, "Credential removed successfully", credential_id = full_id);
                        Ok(false)
                    }
                    None => {
                        Err(format!("Failed to remove credential with ID '{}'", full_id))
                    }
                }
            }
            _ => {
                println!("Multiple credentials match '{}'. Please be more specific:", self.credential_id);
                for cred in matching_credentials {
                    println!("  {} - {} ({})", cred.id, cred.username, cred.source);
                }
                Ok(false)
            }
        }
    }
}
