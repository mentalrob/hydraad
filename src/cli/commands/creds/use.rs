use clap::Args;

use crate::{app::App, cli::commands::Command, data::credential::AuthType, stores::credentials_store::CredentialFilter};

#[derive(Debug, Args, Clone)]
pub struct UseArgs {
    /// The name of the credential to use
    #[arg(required = true, help = "The name of the credential to use")]
    pub name: String,

    #[arg(short, long, value_enum, default_value_t = AuthType::Password)]
    pub auth_type: AuthType,
}

impl Command for UseArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let filter = CredentialFilter {
            username: Some(self.name.clone()),
            auth_type: Some(self.auth_type.clone().into()),
            ..Default::default()
        };
        let credentials = app.credential_storage().filter_credentials(&filter);
        if credentials.is_empty() {
            return Err(format!("No credentials found for {}", self.name));
        }
        let credential = credentials.first().unwrap().clone();
        app.set_current_creds(Some(credential));
        Ok(false)
    }
}
