use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::DomainController;

#[derive(Serialize, Deserialize)]
pub struct DomainControllerStore {
    pub domain_controllers: HashMap<String, DomainController>,
}

impl DomainControllerStore {
    pub fn new() -> Self {
        Self {
            domain_controllers: HashMap::new(),
        }
    }
    
    pub fn add_domain_controller(&mut self, domain_controller: DomainController) {
        self.domain_controllers.insert(domain_controller.domain_name.clone(), domain_controller);
    }
    
    pub fn get_domain_controller_mut(&mut self, domain_name: &str) -> Result<&mut DomainController, String> {
        self.domain_controllers.get_mut(domain_name).ok_or("Domain controller not found".to_string())
    }

    pub fn remove_domain_controller(&mut self, domain_name: &str) {
        self.domain_controllers.remove(domain_name);
    }
    
    pub fn get_domain_controller(&self, domain_name: &str) -> Option<&DomainController> {
        self.domain_controllers.get(domain_name)
    }
    
    pub fn list_domain_controllers(&self) -> Vec<&DomainController> {
        self.domain_controllers.values().collect()
    }
}