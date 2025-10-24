pub mod domain_controller;
pub mod credential;

// Re-export main structs and enums for easier access
pub use domain_controller::{DomainController};
pub use credential::{Credential, AuthData, CredentialType, CredentialStatus};