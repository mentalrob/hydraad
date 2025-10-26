use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, time::Duration};

use anyhow::anyhow;
use trust_dns_resolver::{config::{NameServerConfig, NameServerConfigGroup, Protocol, ResolverConfig, ResolverOpts}, Name, TokioAsyncResolver};

pub async fn dig_srv_short(
    server: String,
    port: u16,
    domain: String,
) -> anyhow::Result<String> {
    // Parse the server IP address
    let server_ip: IpAddr = server.parse()?;
    
    // Create a custom resolver configuration pointing to the specified DNS server
    let mut config = ResolverConfig::new();
    let name_server = NameServerConfig {
        socket_addr: SocketAddr::new(server_ip, port),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: true,
        bind_addr: None,
    };
    config.add_name_server(name_server);
    
    // Set resolver options
    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(5);
    
    // Create the resolver
    let resolver = TokioAsyncResolver::tokio(config, opts);
    
    // Query for SRV records
    let response = resolver.srv_lookup(domain).await?;

    

    let mut fqdn = response.iter().map(|srv| srv.target().to_string()).collect::<Vec<_>>().first().ok_or(anyhow::anyhow!("No domain found !"))?.clone();
    fqdn = fqdn.strip_suffix(".").unwrap().to_string();
    

    Ok(fqdn)
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_dig_srv_short() {
        let result = super::dig_srv_short("10.10.11.76".to_string(), 53, "_kerberos._tcp.voleur.htb".to_string()).await;        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}