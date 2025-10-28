use std::{
    net::{IpAddr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

use clap::Args;
use smb::{
    Client, ClientConfig, ConnectionConfig, FileAccessMask, FileCreateArgs, UncPath,
    connection::AuthMethodsConfig,
};
use sspi::{AuthIdentity, Secret, Username};

use crate::{
    app::App,
    cli::commands::Command,
    data::{AuthData, DomainController},
    utils::dns_operations::dig_srv_short,
};

#[derive(Debug, Args)]
pub struct SharesArgs {
    /// List all shares recursively
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub all: bool,

    /// Specify one share to list
    #[arg(long)]
    pub share: Option<String>,
}

impl Command for SharesArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let (dc, creds) = app.get_current_context()?;
        let client = Client::new(ClientConfig {
            connection: ConnectionConfig {
                auth_methods: AuthMethodsConfig {
                    ntlm: false,
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

        let fqdn = dig_srv_short(
            dc.ip_address.to_string(),
            53,
            format!("_kerberos._tcp.{}", dc.domain_name),
        )
        .await
        .map_err(|e| e.to_string())?;

        let connection = client
            .connect_to_address(&fqdn, socket_addr)
            .await
            .map_err(|e| e.to_string())?;
        println!("Connection established");
        if let AuthData::Password(pass) = creds.auth_data {
            let smb_main_path = format!("{}:445", dc.ip_address.to_string());
            let username = Username::new(creds.username.as_str(), Some(&dc.domain_name.as_str()))
                .map_err(|e| e.to_string())?;
            let identity = AuthIdentity {
                username,
                password: pass.clone().into(),
            };
            let _ = connection
                .authenticate(identity.clone())
                .await
                .map_err(|e| e.to_string())?;
            if self.share.is_none() {
                client
                    ._ipc_connect(&smb_main_path, &identity)
                    .await
                    .map_err(|e| e.to_string())?;

                println!("Available shares on the target:");
                let shares = client
                    .list_shares(&smb_main_path)
                    .await
                    .map_err(|e| e.to_string())?;
                for share in shares.clone() {
                    println!("  - {}", **share.netname.as_ref().unwrap());
                }
                if self.all {
                    for share in shares {
                        let share_name = share.netname.as_ref().unwrap().to_string();
                        println!("\n=== [{}] ===", share_name);
                        // WTF Sen niye çalışmıon aq
                        if share_name.ends_with('$') {
                            continue;
                        }
                        let unc_path = format!(r"\\{}\{}", smb_main_path, share_name);
                        list_recursive(&client, &unc_path, "", &format!(r"{}\{}", dc.domain_name.clone(), creds.username.clone()), &pass, 1).await?;
                    }
                }
            }

            if let Some(ref share_name) = self.share {
                println!("Listing content of share: {}", share_name);
                let unc_path = format!(r"\\{}\{}", smb_main_path, share_name);
                list_recursive(&client, &unc_path, "", &format!(r"{}\{}", dc.domain_name.clone(), creds.username.clone()), &pass, 0).await?;
            }
        }

        // client.ipc_connect(server, username, password)

        async fn list_recursive(
            client: &smb::Client,
            unc_base: &str,       // e.g. \\10.10.11.76\Documents
            path: &str,           // relative path ("" or "Subfolder")
            username_with_domain: &str, // credentials (not used here, but kept for future auth reconnects)
            password: &str,
            depth: usize,
        ) -> Result<(), String> {
            // Construct full UNC path
            let full_unc = if path.is_empty() {
                unc_base.to_string()
            } else {
                format!(r"{}\{}", unc_base, path)
            };



            // Try to open this path (folder or file)
            let unc_path = UncPath::from_str(&full_unc).map_err(|e| e.to_string())?;
            client
                .share_connect(&unc_path, username_with_domain, password.to_string())
                .await
                .map_err(|e| e.to_string())?;

            println!("Share connected");

            match client
                .create_file(
                    &unc_path,
                    &FileCreateArgs::make_open_existing(
                        FileAccessMask::new().with_generic_read(true),
                    ),
                )
                .await
            {
                Ok(_) => {
                    println!("{}📁 {}", "  ".repeat(depth), full_unc);
                }
                Err(e) => {
                    return Err(format!("Failed to open {}: {}", full_unc, e));
                }
            }

            Ok(())
        }
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

    use smb::{Client, ClientConfig, ConnectionConfig, UncPath, connection::AuthMethodsConfig};
    use sspi::{AuthIdentity, Secret, Username};

    #[tokio::test]
    async fn test_shares() {
        let client = Client::new(ClientConfig {
            connection: ConnectionConfig {
                client_name: Some("voleur.htb".to_string()),
                auth_methods: AuthMethodsConfig {
                    ntlm: false,
                    kerberos: true,
                },
                ..Default::default()
            },
            ..Default::default()
        });

        let ip_address = "10.10.11.76".parse::<Ipv4Addr>().unwrap();
        let socket_addr = SocketAddr::V4(SocketAddrV4::new(ip_address, 0));

        unsafe {
            std::env::set_var("SSPI_KDC_URL", "tcp://voleur.htb:88");
        }

        let connection = client
            .connect_to_address("voleur.htb", socket_addr)
            .await
            .unwrap();
        println!("Connection established");
        let username = Username::new("ryan.naylor", Some("voleur.htb")).unwrap();
        let identity = AuthIdentity {
            username,
            password: "HollowOct31Nyt".to_string().into(),
        };
        let session = connection.authenticate(identity).await.unwrap();
        let unc_base = format!(r"\\{}\", "voleur.htb");
        let path = "IT";
        let full_unc = if path.is_empty() {
            unc_base.to_string()
        } else {
            format!(r"{}\{}", unc_base, path)
        };

        let result = client
            .create_file(
                &full_unc.parse().unwrap(),
                &smb::FileCreateArgs::make_open_existing(
                    smb::FileAccessMask::new().with_generic_read(true),
                ),
            )
            .await;
        print!(r"{}", result.is_ok());
        println!("Bişeyler yaşandı amk");
    }
}
