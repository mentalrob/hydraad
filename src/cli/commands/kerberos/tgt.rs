use ascii::AsciiString;
use base64::{prelude::BASE64_STANDARD, Engine};
use clap::Args;
use himmelblau_kerbeiros::TgtRequester;
use himmelblau_kerberos_crypto::Key;

use crate::{app::App, cli::commands::Command, data::AuthData};

#[derive(Debug, Args, Clone)]
pub struct TgtArgs;

impl Command for TgtArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        let (dc, creds) = app.get_current_context()?;
        // Prepare the arguments

        let user_key = match &creds.auth_data {
            crate::data::AuthData::Password(pass) => Key::from_rc4_key_string(ntlm_hash::ntlm_hash(pass.clone().as_str()).as_str()),
            crate::data::AuthData::NtlmHash(hash) => Key::from_rc4_key_string(hash.clone().as_str()),
            _ => return Err("Unsupported authentication type".to_string()),
        }.map_err(|e| e.to_string())?;

        let realm = AsciiString::from_ascii(dc.domain_name.clone()).unwrap();
        let kdc_address = dc.ip_address.clone();
        let username = AsciiString::from_ascii(creds.username.clone()).unwrap();

        // Request the TGT
        let tgt_requester = TgtRequester::new(realm, kdc_address);
        let credential = tgt_requester.request(&username, Some(&user_key)).map_err(|e| e.to_string())?;

        winston::log!(info, "TGT Key Retreived !");
        winston::log!(info, "Building ccache...");

        let ccache: himmelblau_kerberos_ccache::Credential = credential.into();


        let ccache_data = BASE64_STANDARD.encode(&ccache.build());

        let mut new_creds = creds.clone();
        new_creds.id = uuid::Uuid::new_v4().to_string();
        new_creds.auth_data = AuthData::KerberosTicket(ccache_data);
        new_creds.source = "Tgt".to_string();
        app.credential_storage().add_credential(new_creds).map_err(|e| e.to_string())?;        
        winston::log!(info, "New credential added to storage !");
        Ok(false)
    }
}
