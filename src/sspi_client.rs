use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, TcpStream, UdpSocket};

use byteorder::{BigEndian, ReadBytesExt};
use sspi::network_client::NetworkClient;
use sspi::{Error, ErrorKind, NetworkProtocol, NetworkRequest};
use url::Url;

use crate::rustls::{install_default_crypto_provider_if_necessary, load_native_certs};



#[derive(Debug, Clone)]
pub struct ReqwestNetworkClient {
    ip_addr: Ipv4Addr,
}

impl ReqwestNetworkClient {
    pub fn new(ip_addr: Ipv4Addr) -> Self {
        Self { ip_addr }
    }

    fn send_tcp(&self, url: &Url, data: &[u8]) -> Result<Vec<u8>, Error> {
        let addr = format!(
            "{}:{}",
            self.ip_addr.to_string(),
            url.port().unwrap_or(88)
        );
        let mut stream = TcpStream::connect(addr)
            .map_err(|e| Error::new(ErrorKind::NoAuthenticatingAuthority, format!("{:?}", e)))?;

        stream
            .write(data)
            .map_err(|e| Error::new(ErrorKind::NoAuthenticatingAuthority, format!("{:?}", e)))?;

        let len = stream
            .read_u32::<BigEndian>()
            .map_err(|e| Error::new(ErrorKind::NoAuthenticatingAuthority, format!("{:?}", e)))?;

        let mut buf = vec![0; len as usize + 4];
        buf[0..4].copy_from_slice(&(len.to_be_bytes()));

        stream
            .read_exact(&mut buf[4..])
            .map_err(|e| Error::new(ErrorKind::NoAuthenticatingAuthority, format!("{:?}", e)))?;

        Ok(buf)
    }

    fn send_udp(&self, url: &Url, data: &[u8]) -> Result<Vec<u8>, Error> {
        let port = portpicker::pick_unused_port()
            .ok_or_else(|| Error::new(ErrorKind::InternalError, "No free ports"))?;
        let udp_socket = UdpSocket::bind((IpAddr::V4(Ipv4Addr::LOCALHOST), port))?;

        let addr = format!(
            "{}:{}",
            self.ip_addr.to_string(),
            url.port().unwrap_or(88)
        );
        udp_socket.send_to(data, addr)?;

        // 48 000 bytes: default maximum token len in Windows
        let mut buf = vec![0; 0xbb80];

        let n = udp_socket.recv(&mut buf)?;

        let mut reply_buf = Vec::with_capacity(n + 4);
        reply_buf.extend_from_slice(&(n as u32).to_be_bytes());
        reply_buf.extend_from_slice(&buf[0..n]);

        Ok(reply_buf)
    }

    fn send_http(&self, url: &Url, data: &[u8]) -> Result<Vec<u8>, Error> {
        install_default_crypto_provider_if_necessary().map_err(|()| {
            Error::new(
                ErrorKind::SecurityPackageNotFound,
                "failed to install the default crypto provider for TLS",
            )
        })?;

        let client = load_native_certs(reqwest::blocking::ClientBuilder::new())
            .build()
            .map_err(|e| {
                Error::new(
                    ErrorKind::NoAuthenticatingAuthority,
                    format!("failed to build reqwest client: {e}"),
                )
            })?;

        let response = client
            .post(url.clone())
            .body(data.to_vec())
            .send()
            .map_err(|err| match err {
                err if err.to_string().to_lowercase().contains("certificate") => Error::new(
                    ErrorKind::CertificateUnknown,
                    format!("Invalid certificate data: {:?}", err),
                ),
                _ => Error::new(
                    ErrorKind::NoAuthenticatingAuthority,
                    format!("Unable to send the data to the KDC Proxy: {:?}", err),
                ),
            })?
            .error_for_status()
            .map_err(|err| {
                Error::new(
                    ErrorKind::NoAuthenticatingAuthority,
                    format!("KDC Proxy: {err}"),
                )
            })?;

        let body = response.bytes().map_err(|err| {
            Error::new(
                ErrorKind::NoAuthenticatingAuthority,
                format!(
                    "Unable to read the response data from the KDC Proxy: {:?}",
                    err
                ),
            )
        })?;

        // The type bytes::Bytes has a special From implementation for Vec<u8>.
        let body = Vec::from(body);

        Ok(body)
    }
}

impl NetworkClient for ReqwestNetworkClient {
    fn send(&self, request: &NetworkRequest) -> Result<Vec<u8>, Error> {
        match request.protocol {
            NetworkProtocol::Tcp => self.send_tcp(&request.url, &request.data),
            NetworkProtocol::Udp => self.send_udp(&request.url, &request.data),
            NetworkProtocol::Http | NetworkProtocol::Https => {
                self.send_http(&request.url, &request.data)
            }
        }
    }
}
