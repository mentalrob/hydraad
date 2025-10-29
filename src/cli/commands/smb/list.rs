use std::net::{IpAddr, SocketAddr, SocketAddrV4};

use clap::Args;
use smb::{Client, ClientConfig, ConnectionConfig, UncPath, connection::AuthMethodsConfig};

use crate::{app::App, cli::commands::Command};

#[derive(Debug, Args)]
pub struct ListArgs {
    pub path: String,
}


impl Command for ListArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let (dc, creds) = app.get_current_context()?;
        let client = Client::new(ClientConfig {
            connection: ConnectionConfig {
                auth_methods: AuthMethodsConfig {
                    ntlm: true,
                    kerberos: true,
                },
                ..Default::default()
            },
            ..Default::default()
        });

        unsafe {
            std::env::set_var(
                "SSPI_KDC_URL",
                format!("tcp://{}:88", dc.domain_name.clone()),
            );
        }

        let ip_address = dc.ip_address.clone();
        let socket_addr = if let IpAddr::V4(ip) = ip_address {
            SocketAddr::V4(SocketAddrV4::new(ip, 445))
        } else {
            return Err("Unsupported IP address type".to_string());
        };


        Ok(false)
    }
}