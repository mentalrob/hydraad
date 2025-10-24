use clap::Args;
use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement};

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter by domain
    #[arg(short, long)]
    pub domain: Option<String>,
    
    /// Filter by username
    #[arg(short, long)]
    pub username: Option<String>,
    
    /// Show only validated credentials
    #[arg(short, long)]
    pub validated_only: bool,
    
    /// Filter by source
    #[arg(short, long)]
    pub source: Option<String>,
}

impl Command for ListArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let credentials = if self.domain.is_some() || self.username.is_some() || self.validated_only || self.source.is_some() {
            // Apply filters
            let filter = crate::stores::credentials_store::CredentialFilter {
                domain: self.domain.clone(),
                username: self.username.clone(),
                credential_type: None,
                source: self.source.clone(),
                validated_only: self.validated_only,
                has_privileges: None,
            };
            app.credential_storage().filter_credentials(&filter)
        } else {
            // Get all credentials
            app.credential_storage().get_all_credentials()
        };
        
        if credentials.is_empty() {
            println!("No credentials found.");
            return Ok(false);
        }

        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("ID").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Username").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Domain").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Type").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Auth Type").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Source").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Validated").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Last Used").add_attribute(Attribute::Bold).fg(Color::Cyan),
            ]);

        for cred in credentials {
            let validated_cell = if cred.is_validated {
                Cell::new("Yes").fg(Color::Green).add_attribute(Attribute::Bold)
            } else {
                Cell::new("No").fg(Color::Red)
            };

            let last_used = cred.last_used
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "Never".to_string());

            // Truncate ID for display
            let short_id = if cred.id.len() > 8 {
                format!("{}...", &cred.id[..8])
            } else {
                cred.id.clone()
            };

            table.add_row(vec![
                Cell::new(short_id),
                Cell::new(&cred.username),
                Cell::new(&cred.domain),
                Cell::new(format!("{:?}", cred.credential_type)),
                Cell::new(cred.auth_data_type()),
                Cell::new(&cred.source),
                validated_cell,
                Cell::new(last_used),
            ]);
        }

        println!("{}", table);
        Ok(false)
    }
}
