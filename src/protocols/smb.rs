use std::net::{IpAddr, SocketAddr, SocketAddrV4};

use smb::{Client, ClientConfig, ConnectionConfig, connection::AuthMethodsConfig};
use smb_rpc::interface::ShareInfo1;
use sspi::{AuthIdentity, Secret, Username};

use crate::{
    data::{AuthData, Credential, DomainController},
    utils::dns_operations::dig_srv_short,
};

#[derive(PartialEq)]
pub enum SmbAuthType {
    Kerberos,
    Ntlm,
    Both,
}

pub struct SmbClient {
    dc: DomainController,
    creds: Option<Credential>,
    auth_type: SmbAuthType,

    client: Client,
    fqdn: String,
    smb_port: u16,
}

impl SmbClient {
    pub async fn new(
        dc: DomainController,
        creds: Option<Credential>,
        auth_type: SmbAuthType,
        smb_port: u16,
    ) -> Result<Self, String> {
        let client = Client::new(ClientConfig {
            connection: ConnectionConfig {
                auth_methods: AuthMethodsConfig {
                    ntlm: auth_type == SmbAuthType::Ntlm || auth_type == SmbAuthType::Both,
                    kerberos: auth_type == SmbAuthType::Kerberos || auth_type == SmbAuthType::Both,
                },
                ..Default::default()
            },
            ..Default::default()
        });

        let fqdn = dig_srv_short(
            dc.ip_address.to_string(),
            53,
            format!("_kerberos._tcp.{}", dc.domain_name),
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(Self {
            dc,
            creds,
            auth_type,
            client,
            fqdn,
            smb_port,
        })
    }

    fn get_upn_name(&self) -> Result<Username, String> {
        if let Some(cred) = &self.creds {
            Username::parse(&format!(r"{}\{}", self.dc.domain_name, cred.username))
                .map_err(|e| e.to_string())
        } else {
            Username::parse(&"").map_err(|e| e.to_string())
        }
    }

    fn get_upn_name_str(&self) -> String {
        if let Some(cred) = &self.creds {
            format!(r"{}\{}", self.dc.domain_name, cred.username)
        } else {
            "".to_string()
        }
    }

    fn get_password(&self) -> Result<Secret<String>, String> {
        if let Some(cred) = &self.creds
            && let AuthData::Password(pass) = &cred.auth_data
        {
            Ok(pass.clone().into())
        } else {
            Ok(Secret::new("".to_string()))
        }
    }

    fn build_sspi_identity(&self) -> Result<AuthIdentity, String> {
        let username = self.get_upn_name()?;
        let password = self.get_password()?;
        Ok(AuthIdentity { username, password })
    }

    fn get_main_path(&self) -> String {
        format!("{}:{}", self.dc.ip_address.to_string(), self.smb_port)
    }

    pub async fn connect(&self) -> Result<(), String> {
        let ip_address = self.dc.ip_address.clone();
        let socket_addr = if let IpAddr::V4(ip) = ip_address {
            SocketAddr::V4(SocketAddrV4::new(ip, 445))
        } else {
            return Err("Unsupported IP address type".to_string());
        };

        let connection = self
            .client
            .connect_to_address(&self.fqdn, socket_addr)
            .await
            .map_err(|e| e.to_string())?;
        println!("Connection established");

        let identity = self.build_sspi_identity()?;

        let _ = connection
            .authenticate(identity.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn list_shares(&self) -> Result<Vec<ShareInfo1>, String> {
        let _ = self
            .client
            ._ipc_connect(&self.get_main_path(), &self.build_sspi_identity()?)
            .await
            .map_err(|e| e.to_string())?;

        println!("Available shares on the target:");
        let shares = self
            .client
            .list_shares(&self.get_main_path())
            .await
            .map_err(|e| e.to_string())?;
        Ok(shares)
    }
}
