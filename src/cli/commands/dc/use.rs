use clap::Args;

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct UseArgs {
    /// Domain name of the domain controller to use
    pub domain_name: String,
}

impl Command for UseArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        // Get the domain controller from storage
        let dc = app.domain_controller_storage
            .get_domain_controller(&self.domain_name)
            .ok_or_else(|| format!("Domain controller '{}' not found", self.domain_name))?
            .clone();
        
        // Clone it and set as current (this also updates the prompt)
        
        app.set_current_dc(Some(dc));
        
        winston::log!(info, "Done");
        
        Ok(false)
    }
}