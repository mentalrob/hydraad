use std::net::{IpAddr, SocketAddr, SocketAddrV4};

use clap::Args;
use smb::{connection::AuthMethodsConfig, Client, ClientConfig, ConnectionConfig, UncPath};
use sspi::{AuthIdentity, Secret, Username};

use crate::{app::App, cli::commands::Command, data::AuthData, utils::dns_operations::dig_srv_short};

#[derive(Debug,Args)]
pub struct SharesArgs {
    
}

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

        unsafe { std::env::set_var("SSPI_KDC_URL", format!("tcp://{}:88", dc.domain_name.clone())); }

        let ip_address = dc.ip_address.clone();
        let socket_addr = if let IpAddr::V4(ip) = ip_address {
            SocketAddr::V4(SocketAddrV4::new(ip, 0))
        }else{
            return Err("Unsupported IP address type".to_string());
        };

        let fqdn = dig_srv_short(dc.ip_address.to_string(), 53, format!("_kerberos._tcp.{}", dc.domain_name)).await.map_err(|e| e.to_string())?;

        let connection = client.connect_to_address(&fqdn, socket_addr).await.map_err(|e| e.to_string())?;
        println!("Connection established");
        if let AuthData::Password(pass) =  creds.auth_data {
            let username = Username::new(creds.username.as_str(), Some(dc.domain_name.as_str())).map_err(|e| e.to_string())?;
            let identity = AuthIdentity { username, password: pass.into() };
            let _ = connection.authenticate(identity).await.map_err(|e| e.to_string())?;

            let shares = client.list_shares(&fqdn).await.map_err(|e| e.to_string())?;
            println!("Shares: {:#?}", shares);

        }


        // client.ipc_connect(server, username, password)

        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

    use smb::{connection::AuthMethodsConfig, Client, ClientConfig, ConnectionConfig, UncPath};
    use sspi::{AuthIdentity, Secret, Username};

    #[tokio::test]
    async fn test_shares() {
        let client = Client::new(ClientConfig {
            connection: ConnectionConfig {
                client_name: Some("voleur.htb".to_string()),
                auth_methods: AuthMethodsConfig {
                    ntlm: false,
                    kerberos: true
                },
                ..Default::default()  
            },
            ..Default::default()
        });

        let ip_address = "10.10.11.76".parse::<Ipv4Addr>().unwrap();
        let socket_addr = SocketAddr::V4(SocketAddrV4::new(ip_address, 0));

        unsafe { std::env::set_var("SSPI_KDC_URL", "tcp://voleur.htb:88"); }

        let connection = client.connect_to_address("voleur.htb", socket_addr).await.unwrap();
        println!("Connection established");
        let username = Username::new("ryan.naylor", Some("voleur.htb")).unwrap();
        let identity = AuthIdentity { username, password: "HollowOct31Nyt".to_string().into() };
        let session = connection.authenticate(identity).await.unwrap();

        println!("Bişeyler yaşandı amk");
    }
}