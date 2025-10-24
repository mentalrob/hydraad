use std::net::IpAddr;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DomainController {
    /// IP address of the domain controller
    pub ip_address: IpAddr,
    
    /// Domain name this DC serves
    pub domain_name: String,
    
    
    /// LDAP port (usually 389 or 636 for LDAPS)
    pub ldap_port: u16,
    
    /// Global Catalog port (usually 3268 or 3269 for GC SSL)
    pub gc_port: u16,
    
    /// Whether LDAPS is enabled
    pub ldaps_enabled: bool,
    
}
impl DomainController {
    /// Create a new DomainController instance
    pub fn new(
        ip_address: IpAddr,
        domain_name: String,
    ) -> Self {
        Self {
            ip_address,
            domain_name,
            ldap_port: 389,
            gc_port: 3268,
            ldaps_enabled: false,
        }
    }
    
    /// Get the LDAP connection string
    pub fn ldap_url(&self) -> String {
        let protocol = if self.ldaps_enabled { "ldaps" } else { "ldap" };
        let port = if self.ldaps_enabled { 636 } else { self.ldap_port };
        format!("{}://{}:{}", protocol, self.ip_address, port)
    }
    
    /// Get the Global Catalog connection string
    pub fn gc_url(&self) -> String {
        let port = if self.ldaps_enabled { 3269 } else { self.gc_port };
        format!("ldap://{}:{}", self.ip_address, port)
    }
}