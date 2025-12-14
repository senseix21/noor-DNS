// Upstream DoT (DNS-over-TLS) client
// Forward DNS queries to upstream DoT servers (1.1.1.1:853, etc)

use anyhow::{Context, Result};
use hickory_proto::op::Message;
use rustls::pki_types::ServerName;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

#[derive(Clone)]
pub struct DotClient {
    connector: TlsConnector,
    upstream_addr: SocketAddr,
    server_name: ServerName<'static>,
}

impl DotClient {
    pub fn new(upstream_addr: SocketAddr) -> Result<Self> {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let connector = TlsConnector::from(Arc::new(config));
        
        // Use IP as server name (Cloudflare accepts this)
        let server_name = ServerName::try_from("cloudflare-dns.com")
            .map_err(|_| anyhow::anyhow!("Invalid server name"))?
            .to_owned();

        Ok(Self {
            connector,
            upstream_addr,
            server_name,
        })
    }

    pub async fn query(&self, msg: &Message) -> Result<Message> {
        // Connect
        let stream = TcpStream::connect(self.upstream_addr).await
            .context("Failed to connect to upstream DoT server")?;
        
        let mut tls_stream = self.connector.connect(self.server_name.clone(), stream).await
            .context("TLS handshake with upstream failed")?;

        // Send query (RFC 7858: 2-byte length + DNS message)
        let query_bytes = msg.to_vec()?;
        let len = query_bytes.len() as u16;
        
        tls_stream.write_u16(len).await?;
        tls_stream.write_all(&query_bytes).await?;
        tls_stream.flush().await?;

        // Read response
        let response_len = tls_stream.read_u16().await? as usize;
        let mut response_bytes = vec![0u8; response_len];
        tls_stream.read_exact(&mut response_bytes).await?;

        Message::from_vec(&response_bytes)
            .context("Failed to parse upstream DoT response")
    }
}
