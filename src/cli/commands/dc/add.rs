use std::net::{IpAddr, Ipv4Addr};

use clap::{Args, arg};
use ldap3::{LdapConn, LdapConnAsync, Scope, SearchEntry};
use log::info;

use crate::{app::App, cli::commands::Command, data::DomainController};

#[derive(Debug, Args)]
pub struct AddArgs {
    pub ip: Ipv4Addr,
    #[arg(long, default_value_t = false)]
    pub ldaps: bool,
    #[arg(long, default_value_t = 389)]
    pub ldap_port: u16,
    #[arg(long)]
    pub domain: Option<String>,
}

impl Command for AddArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let mut dc = DomainController::new(IpAddr::V4(self.ip), "UNKNOWN".to_string());

        dc.ldaps_enabled = self.ldaps;
        dc.ldap_port = self.ldap_port;

        let ldap_url = dc.ldap_url();

        let (conn, mut ldap) = LdapConnAsync::new(ldap_url.as_str())
            .await
            .map_err(|e| e.to_string())?;
        ldap3::drive!(conn);
        let (rs, _res) = ldap
            .search(
                "",
                Scope::Base,
                "(objectClass=*)",
                vec!["defaultNamingContext", "namingContexts"],
            )
            .await
            .map_err(|e| e.to_string())?
            .success()
            .map_err(|e| e.to_string())?;

        if let Some(domain) = &self.domain {
            dc.domain_name = domain.clone();
        } else {
            for entry in rs {
                let search_entry = SearchEntry::construct(entry);
                let default_naming_context = search_entry
                    .attrs
                    .get("defaultNamingContext")
                    .or(search_entry.attrs.get("namingContexts"))
                    .ok_or("Failed to get defaultNamingContext or namingContexts".to_string())?;
                let default_naming_context = default_naming_context
                    .get(0)
                    .ok_or("Failed to get defaultNamingContext or namingContexts".to_string())?;
                dc.domain_name = default_naming_context
                    .split(",")
                    .filter_map(|f| f.strip_prefix("DC="))
                    .map(|f| f.to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    .join(".");
                println!("Domain name found {}", dc.domain_name.clone());
            }
        }
        ldap.unbind().await.map_err(|e| e.to_string())?;

        // Add the domain controller to storage
        app.domain_controller_storage
            .add_domain_controller(dc.clone());
        println!("New dc added");
        Ok(false)
    }
}
