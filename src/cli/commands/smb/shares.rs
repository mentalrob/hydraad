use std::net::{IpAddr, SocketAddr, SocketAddrV4};

use clap::Args;
use smb::{connection::AuthMethodsConfig, Client, ClientConfig, ConnectionConfig, UncPath};
use sspi::{AuthIdentity, Secret, Username};

use crate::{app::App, cli::commands::Command, data::AuthData};

#[derive(Debug,Args)]
pub struct SharesArgs;

impl Command for SharesArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let (dc, creds) = app.get_current_context()?;
        let client = Client::new(ClientConfig {
            connection: ConnectionConfig {
                auth_methods: AuthMethodsConfig {
                    ntlm: false,
                    kerberos: true
                },
                ..Default::default()  
            },
            ..Default::default()
        });

        let ip_address = dc.ip_address.clone();
        let socket_addr = if let IpAddr::V4(ip) = ip_address {
            SocketAddr::V4(SocketAddrV4::new(ip, 0))
        }else{
            return Err("Unsupported IP address type".to_string());
        };

        let connection = client.connect_to_address(&dc.domain_name, socket_addr).await.map_err(|e| e.to_string())?;
        println!("Connection established");
        if let AuthData::Password(pass) =  creds.auth_data {
            let username = Username::new(creds.username.as_str(), Some(dc.domain_name.as_str())).map_err(|e| e.to_string())?;
            let identity = AuthIdentity { username, password: pass.into() };
            let session = connection.authenticate(identity).await.map_err(|e| e.to_string())?;

            println!("Bişeyler yaşandı amk")

        }


        // client.ipc_connect(server, username, password)

        Ok(false)
    }
}

