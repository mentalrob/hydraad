use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum AuthType {
    Password,
    NtlmHash,
    LmHash,
    Tgt,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CredType {
    DomainUser,
    LocalAdmin,
    DomainAdmin,
    EnterpriseAdmin,
    ServiceAccount,
    MachineAccount,
    ManagedServiceAccount,
    GroupManagedServiceAccount,
    ServicePrincipal,
    BuiltIn,
    Unknown,
}

impl From<CredType> for CredentialType {
    fn from(cred_type: CredType) -> Self {
        match cred_type {
            CredType::DomainUser => CredentialType::DomainUser,
            CredType::LocalAdmin => CredentialType::LocalAdmin,
            CredType::DomainAdmin => CredentialType::DomainAdmin,
            CredType::EnterpriseAdmin => CredentialType::EnterpriseAdmin,
            CredType::ServiceAccount => CredentialType::ServiceAccount,
            CredType::MachineAccount => CredentialType::MachineAccount,
            CredType::ManagedServiceAccount => CredentialType::ManagedServiceAccount,
            CredType::GroupManagedServiceAccount => CredentialType::GroupManagedServiceAccount,
            CredType::ServicePrincipal => CredentialType::ServicePrincipal,
            CredType::BuiltIn => CredentialType::BuiltIn,
            CredType::Unknown => CredentialType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Credential {
    /// Unique identifier for this credential
    pub id: String,
    
    /// Username (can be UPN, SAM, or DN format)
    pub username: String,
    
    /// Authentication data
    pub auth_data: AuthData,
    
    /// Credential type/source
    pub credential_type: CredentialType,
    
    /// Privilege level or group memberships
    pub privileges: Vec<String>,
    
    /// Whether this credential has been validated
    pub is_validated: bool,
    
    /// Last time this credential was successfully used
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    
    /// When this credential was discovered/obtained
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    
    /// Source of this credential (e.g., "mimikatz", "secretsdump", "manual")
    pub source: String,
    
    /// Target domain controller where this was obtained/tested
    pub target_dc: Option<String>,
    
    /// Additional notes or metadata
    pub notes: Option<String>,
    
    /// Additional metadata for red team operations
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthData {
    /// Plain text password
    Password(String),
    
    /// NTLM hash
    NtlmHash(String),
    
    /// LM hash (legacy)
    LmHash(String),
    
    /// Both LM and NTLM hashes
    LmNtlm { lm: String, ntlm: String },
    
    /// Kerberos ticket (base64 encoded)
    KerberosTicket(String),
    
    /// Certificate for PKINIT
    Certificate {
        cert_data: String,
        private_key: Option<String>,
    },
    
    /// Token or session data
    Token(String),
    
    /// Custom authentication data
    Custom(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CredentialType {
    /// Regular domain user
    DomainUser,
    
    /// Local administrator
    LocalAdmin,
    
    /// Domain administrator
    DomainAdmin,
    
    /// Enterprise administrator
    EnterpriseAdmin,
    
    /// Service account
    ServiceAccount,
    
    /// Computer/machine account
    MachineAccount,
    
    /// Managed Service Account (MSA)
    ManagedServiceAccount,
    
    /// Group Managed Service Account (gMSA)
    GroupManagedServiceAccount,
    
    /// Kerberos Service Principal Name
    ServicePrincipal,
    
    /// Built-in account (like krbtgt)
    BuiltIn,

    Unknown,
    
    /// Unknown or unclassified
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CredentialStatus {
    /// Credential is active and working
    Active,
    
    /// Credential is expired
    Expired,
    
    /// Account is disabled
    Disabled,
    
    /// Account is locked out
    LockedOut,
    
    /// Password needs to be changed
    PasswordExpired,
    
    /// Credential validation failed
    Invalid,
    
    /// Status unknown/not tested
    Unknown,
}

impl Credential {
    /// Create a new credential with password authentication
    pub fn new_password(
        username: String,
        password: String,
        source: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            auth_data: AuthData::Password(password),
            credential_type: CredentialType::Unknown,
            privileges: Vec::new(),
            is_validated: false,
            last_used: None,
            discovered_at: chrono::Utc::now(),
            source,
            target_dc: None,
            notes: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a new credential with NTLM hash
    pub fn new_ntlm_hash(
        username: String,
        ntlm_hash: String,
        source: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            auth_data: AuthData::NtlmHash(ntlm_hash),
            credential_type: CredentialType::Unknown,
            privileges: Vec::new(),
            is_validated: false,
            last_used: None,
            discovered_at: chrono::Utc::now(),
            source,
            target_dc: None,
            notes: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Get the full username in UPN format if possible
    pub fn upn(&self, domain: &String) -> String {
        if self.username.contains('@') {
            self.username.clone()
        } else {
            format!("{}@{}", self.username, domain)
        }
    }
    
    /// Get the username in DOMAIN\\username format
    pub fn domain_username(&self, domain: &String) -> String {
        if self.username.contains('\\') {
            self.username.clone()
        } else {
            format!("{}\\{}", domain, self.username)
        }
    }
    
    /// Check if this credential has a specific privilege
    pub fn has_privilege(&self, privilege: &str) -> bool {
        self.privileges.iter().any(|p| p.eq_ignore_ascii_case(privilege))
    }
    
    /// Add a privilege to this credential
    pub fn add_privilege(&mut self, privilege: String) {
        if !self.has_privilege(&privilege) {
            self.privileges.push(privilege);
        }
    }
    
    /// Mark this credential as validated
    pub fn mark_validated(&mut self) {
        self.is_validated = true;
        self.last_used = Some(chrono::Utc::now());
    }
    
    /// Update the last used timestamp
    pub fn update_last_used(&mut self) {
        self.last_used = Some(chrono::Utc::now());
    }
    
    /// Set metadata key-value pair
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
    
    /// Check if this credential contains sensitive authentication data
    pub fn has_sensitive_auth_data(&self) -> bool {
        matches!(
            self.auth_data,
            AuthData::Password(_) | AuthData::Certificate { private_key: Some(_), .. }
        )
    }
    
    /// Get a safe representation of the auth data type (without sensitive data)
    pub fn auth_data_type(&self) -> String {
        match &self.auth_data {
            AuthData::Password(p) => format!("{}", p),
            AuthData::NtlmHash(n) => format!("{}", n),
            AuthData::LmHash(l) => format!("{}", l),
            AuthData::LmNtlm { .. } => "LM/NTLM Hash".to_string(),
            AuthData::KerberosTicket(_) => "Kerberos Ticket".to_string(),
            AuthData::Certificate { .. } => "Certificate".to_string(),
            AuthData::Token(_) => "Token".to_string(),
            AuthData::Custom(_) => "Custom".to_string(),
        }
    }
}