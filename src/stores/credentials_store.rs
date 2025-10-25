use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::data::{credential::AuthType, AuthData, Credential, CredentialStatus, CredentialType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsStore {
    /// Map of credential ID to credential
    credentials: HashMap<String, Credential>,
    
    /// Index by username for quick lookups
    username_index: HashMap<String, Vec<String>>,
    
    /// Index by credential type
    type_index: HashMap<CredentialType, Vec<String>>,
    
    /// Index by source (e.g., "mimikatz", "secretsdump")
    source_index: HashMap<String, Vec<String>>,
    
    /// Statistics
    stats: CredentialStats,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CredentialStats {
    pub total_credentials: usize,
    pub validated_credentials: usize,
    pub by_type: HashMap<CredentialType, usize>,
    pub by_domain: HashMap<String, usize>,
    pub by_source: HashMap<String, usize>,
}

#[derive(Debug, Clone, Default)]
pub struct CredentialFilter {
    pub domain: Option<String>,
    pub username: Option<String>,
    pub credential_type: Option<CredentialType>,
    pub auth_type: Option<AuthType>,
    pub source: Option<String>,
    pub validated_only: bool,
    pub has_privileges: Option<Vec<String>>,
}

impl Default for CredentialsStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsStore {
    /// Create a new empty credentials store
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
            username_index: HashMap::new(),
            type_index: HashMap::new(),
            source_index: HashMap::new(),
            stats: CredentialStats::default(),
        }
    }
    
    /// Add a credential to the store
    pub fn add_credential(&mut self, credential: Credential) -> Result<String, String> {
        let id = credential.id.clone();
        
        // Check for duplicates
        if self.credentials.contains_key(&id) {
            return Err(format!("Credential with ID {} already exists", id));
        }
        
        // Update indices
        self.add_to_username_index(&credential.username, &id);
        self.add_to_type_index(&credential.credential_type, &id);
        self.add_to_source_index(&credential.source, &id);
        
        // Add to main store
        self.credentials.insert(id.clone(), credential);
        
        // Update statistics
        self.update_stats();
        
        Ok(id)
    }
    
    /// Get a credential by ID
    pub fn get_credential(&self, id: &str) -> Option<&Credential> {
        self.credentials.get(id)
    }
    
    /// Get a mutable reference to a credential by ID
    pub fn get_credential_mut(&mut self, id: &str) -> Option<&mut Credential> {
        self.credentials.get_mut(id)
    }
    
    /// Remove a credential by ID
    pub fn remove_credential(&mut self, id: &str) -> Option<Credential> {
        if let Some(credential) = self.credentials.remove(id) {
            // Remove from indices
            self.remove_from_username_index(&credential.username, id);
            self.remove_from_type_index(&credential.credential_type, id);
            self.remove_from_source_index(&credential.source, id);
            
            // Update statistics
            self.update_stats();
            
            Some(credential)
        } else {
            None
        }
    }
    
    /// Update an existing credential
    pub fn update_credential(&mut self, id: &str, updated_credential: Credential) -> Result<(), String> {
        if !self.credentials.contains_key(id) {
            return Err(format!("Credential with ID {} not found", id));
        }
        
        // Remove old credential from indices
        if let Some(old_credential) = self.credentials.get_mut(id).cloned() {
            self.remove_from_username_index(&old_credential.username, id);
            self.remove_from_type_index(&old_credential.credential_type, id);
            self.remove_from_source_index(&old_credential.source, id);
        }
        
        // Add new credential to indices
        self.add_to_username_index(&updated_credential.username, id);
        self.add_to_type_index(&updated_credential.credential_type, id);
        self.add_to_source_index(&updated_credential.source, id);
        
        // Update the credential
        self.credentials.insert(id.to_string(), updated_credential);
        
        // Update statistics
        self.update_stats();
        
        Ok(())
    }
    
    /// Get all credentials
    pub fn get_all_credentials(&self) -> Vec<Credential> {
        self.credentials.values().cloned().collect()
    }
    
   
    
    /// Get credentials by username
    pub fn get_credentials_by_username(&self, username: &str) -> Vec<&Credential> {
        self.username_index
            .get(username)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.credentials.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get credentials by type
    pub fn get_credentials_by_type(&self, credential_type: &CredentialType) -> Vec<&Credential> {
        self.type_index
            .get(credential_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.credentials.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Get credentials by source
    pub fn get_credentials_by_source(&self, source: &str) -> Vec<&Credential> {
        self.source_index
            .get(source)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.credentials.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Filter credentials based on criteria
    pub fn filter_credentials(&self, filter: &CredentialFilter) -> Vec<Credential> {
        self.credentials
            .values()
            .cloned()
            .filter(|cred| {
                // Username filter
                if let Some(ref username) = filter.username {
                    if !cred.username.eq_ignore_ascii_case(username) {
                        return false;
                    }
                }

                if let Some(ref auth_type) = filter.auth_type {
                    return match auth_type {
                        AuthType::Password => matches!(cred.auth_data, AuthData::Password(_)),
                        AuthType::NtlmHash => matches!(cred.auth_data, AuthData::NtlmHash(_)),
                        AuthType::LmHash => matches!(cred.auth_data, AuthData::LmHash(_)),
                        AuthType::Tgt => matches!(cred.auth_data, AuthData::KerberosTicket(_)),
                    };
                }
                
                // Type filter
                if let Some(ref cred_type) = filter.credential_type {
                    if cred.credential_type != *cred_type {
                        return false;
                    }
                }
                
                // Source filter
                if let Some(ref source) = filter.source {
                    if !cred.source.eq_ignore_ascii_case(source) {
                        return false;
                    }
                }
                
                // Validated filter
                if filter.validated_only && !cred.is_validated {
                    return false;
                }
                
                // Privileges filter
                if let Some(ref required_privileges) = filter.has_privileges {
                    for privilege in required_privileges {
                        if !cred.has_privilege(privilege) {
                            return false;
                        }
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Get validated credentials only
    pub fn get_validated_credentials(&self) -> Vec<&Credential> {
        self.credentials
            .values()
            .filter(|cred| cred.is_validated)
            .collect()
    }
    
    /// Get credentials with specific privileges
    pub fn get_credentials_with_privilege(&self, privilege: &str) -> Vec<&Credential> {
        self.credentials
            .values()
            .filter(|cred| cred.has_privilege(privilege))
            .collect()
    }
    
    /// Search credentials by text (username, domain, notes)
    pub fn search_credentials(&self, query: &str) -> Vec<&Credential> {
        let query_lower = query.to_lowercase();
        self.credentials
            .values()
            .filter(|cred| {
                cred.username.to_lowercase().contains(&query_lower)
                    || cred.source.to_lowercase().contains(&query_lower)
                    || cred.notes.as_ref().map_or(false, |n| n.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
    
    /// Mark a credential as validated
    pub fn mark_credential_validated(&mut self, id: &str) -> Result<(), String> {
        if let Some(credential) = self.credentials.get_mut(id) {
            credential.mark_validated();
            self.update_stats();
            Ok(())
        } else {
            Err(format!("Credential with ID {} not found", id))
        }
    }
    
    /// Update last used timestamp for a credential
    pub fn update_credential_last_used(&mut self, id: &str) -> Result<(), String> {
        if let Some(credential) = self.credentials.get_mut(id) {
            credential.update_last_used();
            Ok(())
        } else {
            Err(format!("Credential with ID {} not found", id))
        }
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> &CredentialStats {
        &self.stats
    }
    
    /// Get total number of credentials
    pub fn len(&self) -> usize {
        self.credentials.len()
    }
    
    /// Check if the store is empty
    pub fn is_empty(&self) -> bool {
        self.credentials.is_empty()
    }
    
    /// Clear all credentials
    pub fn clear(&mut self) {
        self.credentials.clear();
        self.username_index.clear();
        self.type_index.clear();
        self.source_index.clear();
        self.stats = CredentialStats::default();
    }
    
    /// Save credentials to a JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    /// Load credentials from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let store: Self = serde_json::from_str(&content)?;
        Ok(store)
    }
    
    /// Export credentials to CSV format (without sensitive data)
    pub fn export_to_csv(&self) -> String {
        let mut csv = String::from("ID,Username,Domain,Type,Source,Validated,Privileges,Last Used,Discovered At\n");
        
        for credential in self.credentials.values() {
            let privileges = credential.privileges.join(";");
            let last_used = credential.last_used
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                .unwrap_or_else(|| "Never".to_string());
            let discovered_at = credential.discovered_at.format("%Y-%m-%d %H:%M:%S UTC");
            
            csv.push_str(&format!(
                "{},{},{:?},{},{},{},{},{}\n",
                credential.id,
                credential.username,
                credential.credential_type,
                credential.source,
                credential.is_validated,
                privileges,
                last_used,
                discovered_at
            ));
        }
        
        csv
    }
    
  
    fn add_to_username_index(&mut self, username: &str, id: &str) {
        self.username_index
            .entry(username.to_lowercase())
            .or_insert_with(Vec::new)
            .push(id.to_string());
    }
    
    fn add_to_type_index(&mut self, credential_type: &CredentialType, id: &str) {
        self.type_index
            .entry(credential_type.clone())
            .or_insert_with(Vec::new)
            .push(id.to_string());
    }
    
    fn add_to_source_index(&mut self, source: &str, id: &str) {
        self.source_index
            .entry(source.to_lowercase())
            .or_insert_with(Vec::new)
            .push(id.to_string());
    }
    
  
    
    fn remove_from_username_index(&mut self, username: &str, id: &str) {
        if let Some(ids) = self.username_index.get_mut(&username.to_lowercase()) {
            ids.retain(|x| x != id);
            if ids.is_empty() {
                self.username_index.remove(&username.to_lowercase());
            }
        }
    }
    
    fn remove_from_type_index(&mut self, credential_type: &CredentialType, id: &str) {
        if let Some(ids) = self.type_index.get_mut(credential_type) {
            ids.retain(|x| x != id);
            if ids.is_empty() {
                self.type_index.remove(credential_type);
            }
        }
    }
    
    fn remove_from_source_index(&mut self, source: &str, id: &str) {
        if let Some(ids) = self.source_index.get_mut(&source.to_lowercase()) {
            ids.retain(|x| x != id);
            if ids.is_empty() {
                self.source_index.remove(&source.to_lowercase());
            }
        }
    }
    
    fn update_stats(&mut self) {
        self.stats.total_credentials = self.credentials.len();
        self.stats.validated_credentials = self.credentials.values().filter(|c| c.is_validated).count();
        
        // Clear existing stats
        self.stats.by_type.clear();
        self.stats.by_domain.clear();
        self.stats.by_source.clear();
        
        // Recalculate stats
        for credential in self.credentials.values() {
            *self.stats.by_type.entry(credential.credential_type.clone()).or_insert(0) += 1;
            *self.stats.by_source.entry(credential.source.clone()).or_insert(0) += 1;
        }
    }
}
