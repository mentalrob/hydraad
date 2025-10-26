use crate::{app::App, cli::commands::Command};
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct TgsArgs {}

impl Command for TgsArgs {
    async fn execute(&self, app: &mut App) -> Result<bool, String> {
        Ok(false)
    }
}

#[cfg(test)]
mod test {

    use base64::Engine;
    use sspi::{
        AcquireCredentialsHandleResult, AuthIdentity, BufferType, ClientRequestFlags, CredentialUse, CredentialsBuffers, DataRepresentation, Kerberos, KerberosConfig, SecurityBuffer, SecurityBufferType, SecurityStatus, Sspi, SspiImpl, Username
    };
    use std::{error::Error, net::Ipv4Addr, str::FromStr};

    use crate::sspi_client::ReqwestNetworkClient;

    fn request_tgt(
        kerberos: &mut Kerberos,
        cred_handle: &mut <Kerberos as SspiImpl>::CredentialsHandle,
        hostname: &str,
        client: &mut ReqwestNetworkClient,
    ) -> anyhow::Result<(String, SecurityStatus)> {
        // For TGT request, use krbtgt service or the target service
        let target_name = format!("krbtgt/{}", hostname);

        // Empty input buffer for initial TGT request
        let mut input_buffer = vec![SecurityBuffer::new(Vec::new(), BufferType::Token)];
        let mut output_buffer = vec![SecurityBuffer::new(Vec::new(), BufferType::Token)];

        let mut builder = kerberos
            .initialize_security_context()
            .with_credentials_handle(cred_handle)
            .with_context_requirements(ClientRequestFlags::MUTUAL_AUTH)
            .with_target_data_representation(DataRepresentation::Native)
            .with_target_name(&target_name)
            .with_input(&mut input_buffer)
            .with_output(&mut output_buffer);

        let result = kerberos
            .initialize_security_context_impl(&mut builder)?
            .resolve_with_client(client)?;

        let output_token =
            base64::engine::general_purpose::STANDARD.encode(&output_buffer[0].buffer);

        Ok((output_token, result.status))
    }

    fn get_cred_handle(
        kerberos: &mut Kerberos,
        identity: AuthIdentity
    ) -> AcquireCredentialsHandleResult<Option<CredentialsBuffers>> {

        kerberos
            .acquire_credentials_handle()
            .with_credential_use(CredentialUse::Outbound)
            .with_auth_data(&identity.into())
            .execute(kerberos)
            .expect("AcquireCredentialsHandle resulted in error")
    }

    #[test]
    fn test_tgs() -> anyhow::Result<()> {
        // Initialize Kerberos security context

        // Set up the credentials for authentication
        let username = "tom"; // Replace with your username
        let password = "Abcd1234"; // Replace with your password
        let domain = "SAMDOM.EXAMPLE.COM"; // Replace with your domain
        let mut client = ReqwestNetworkClient::new("192.168.68.100".parse().unwrap());
        let config = KerberosConfig {
            kdc_url: Some(url::Url::parse(format!("tcp://{}:88", domain).as_str())?),
            client_computer_name: Some(domain.to_string()),
        };
        let mut kerberos = Kerberos::new_client_from_config(config)?;

        // Create identity for authentication
        let identity = sspi::AuthIdentity {
            username: Username::new(username, Some(domain))?,
            password: password.to_string().into(),
        };

        // Acquire credentials handle
        let mut acq_creds_handle_result = get_cred_handle(&mut kerberos, identity);

        // Perform TGT request
        let (output_token, status) = request_tgt(
            &mut kerberos,
            &mut acq_creds_handle_result.credentials_handle,
            domain,
            &mut client,
        )?;

        match status {
            SecurityStatus::Ok | SecurityStatus::ContinueNeeded => {
                println!("TGT request successful!");
                println!("Status: {:?}", status);
                println!("Token (base64): {}", output_token);
                println!(
                    "Token size: {} bytes",
                    base64::engine::general_purpose::STANDARD
                        .decode(&output_token)?
                        .len()
                );
            }
            _ => {
                eprintln!("TGT request failed with status: {:?}", status);
            }
        }

        Ok(())
    }
}
