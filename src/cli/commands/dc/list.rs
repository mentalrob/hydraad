use clap::Args;
use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement};

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct ListArgs {
    // No additional arguments needed for listing
}

impl Command for ListArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let domain_controllers = app.domain_controller_storage.list_domain_controllers();
        
        if domain_controllers.is_empty() {
            println!("No domain controllers found.");
            return Ok(false);
        }

        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Domain Name").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("IP Address").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("LDAP Port").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("LDAPS").add_attribute(Attribute::Bold).fg(Color::Cyan),
                Cell::new("Status").add_attribute(Attribute::Bold).fg(Color::Cyan),
            ]);

        for dc in domain_controllers {
            let status = if let Some(current_dc) = &app.current_used_dc {
                if current_dc.domain_name == dc.domain_name {
                    Cell::new("ACTIVE").fg(Color::Green).add_attribute(Attribute::Bold)
                } else {
                    Cell::new("INACTIVE").fg(Color::Yellow)
                }
            } else {
                Cell::new("INACTIVE").fg(Color::Yellow)
            };

            table.add_row(vec![
                Cell::new(&dc.domain_name),
                Cell::new(dc.ip_address.to_string()),
                Cell::new(dc.ldap_port.to_string()),
                Cell::new(if dc.ldaps_enabled { "Yes" } else { "No" }),
                status,
            ]);
        }

        println!("{}", table);
        Ok(false)
    }
}