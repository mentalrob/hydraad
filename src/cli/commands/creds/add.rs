use clap::{Args, ValueEnum};
use log::info;
use winston::log;

use crate::{app::App, cli::commands::Command, data::{credential::{AuthType, CredType}, AuthData, Credential}};

#[derive(Debug, Args)]
pub struct AddArgs {
    /// Username (can be UPN, SAM, or DN format)
    pub username: String,
    
    /// Authentication type
    #[arg(short, long, value_enum, default_value_t = AuthType::Password)]
    pub auth_type: AuthType,
    
    /// Authentication data (password, hash, etc.)
    pub auth_data: String,
    
    /// Credential type
    #[arg(short = 't', long, value_enum, default_value_t = CredType::DomainUser)]
    pub cred_type: CredType,
    
    /// Source of this credential (e.g., "manual", "mimikatz", "secretsdump")
    #[arg(short, long, default_value = "manual")]
    pub source: String,
    
    /// Additional notes
    #[arg(short, long)]
    pub notes: Option<String>,
}



impl Command for AddArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        // Create the appropriate AuthData based on type
        let auth_data = match self.auth_type {
            AuthType::Password => AuthData::Password(self.auth_data.clone()),
            AuthType::NtlmHash => AuthData::NtlmHash(self.auth_data.clone()),
            AuthType::LmHash => AuthData::LmHash(self.auth_data.clone()),
            AuthType::Tgt => AuthData::KerberosTicket(self.auth_data.clone()),
        };
        
        // Create the credential
        let credential = Credential {
            id: uuid::Uuid::new_v4().to_string(),
            username: self.username.clone(),
            auth_data,
            credential_type: self.cred_type.clone().into(),
            privileges: Vec::new(),
            is_validated: false,
            last_used: None,
            discovered_at: chrono::Utc::now(),
            source: self.source.clone(),
            target_dc: None,
            notes: self.notes.clone(),
            metadata: std::collections::HashMap::new(),
        };
        
        // Add the credential to storage
        match app.credential_storage().add_credential(credential) {
            Ok(id) => {
                println!("Credential added successfully");
                Ok(false)
            }
            Err(e) => Err(format!("Failed to add credential: {}", e))
        }
    }
}
