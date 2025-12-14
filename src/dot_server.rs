// DoT (DNS-over-TLS) server - RFC 7858
// Simple, direct implementation

use crate::tls_config::TlsConfig;
use anyhow::{Context, Result};
use hickory_proto::op::Message;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, server::TlsStream};

const MAX_DNS_MESSAGE_SIZE: usize = 65535;

pub struct DotServer {
    listener: TcpListener,
    acceptor: TlsAcceptor,
}

impl DotServer {
    pub async fn new(bind_addr: SocketAddr) -> Result<Self> {
        log::info!("Generating self-signed TLS certificate for DoT...");
        let tls_config = TlsConfig::generate_self_signed()
            .context("Failed to generate TLS config")?;
        
        let server_config = tls_config.server_config()
            .context("Failed to create TLS server config")?;
        
        let acceptor = TlsAcceptor::from(server_config);
        
        let listener = TcpListener::bind(bind_addr).await
            .context(format!("Failed to bind DoT server to {}", bind_addr))?;
        
        log::info!("DoT server listening on {}", bind_addr);
        
        Ok(Self { listener, acceptor })
    }

    pub async fn accept(&self) -> Result<(TlsStream<TcpStream>, SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;
        let tls_stream = self.acceptor.accept(stream).await
            .context("TLS handshake failed")?;
        Ok((tls_stream, addr))
    }

    // Read DNS message from TLS stream (RFC 7858 format: 2-byte length + message)
    pub async fn read_message(stream: &mut TlsStream<TcpStream>) -> Result<Message> {
        // Read 2-byte length prefix
        let mut len_buf = [0u8; 2];
        stream.read_exact(&mut len_buf).await
            .context("Failed to read message length")?;
        let msg_len = u16::from_be_bytes(len_buf) as usize;

        if msg_len > MAX_DNS_MESSAGE_SIZE {
            anyhow::bail!("DNS message too large: {} bytes", msg_len);
        }

        // Read actual DNS message
        let mut msg_buf = vec![0u8; msg_len];
        stream.read_exact(&mut msg_buf).await
            .context("Failed to read DNS message")?;

        Message::from_vec(&msg_buf)
            .context("Failed to parse DNS message")
    }

    // Write DNS message to TLS stream (with 2-byte length prefix)
    pub async fn write_message(stream: &mut TlsStream<TcpStream>, msg: &Message) -> Result<()> {
        let msg_bytes = msg.to_vec()
            .context("Failed to serialize DNS message")?;
        
        let msg_len = msg_bytes.len() as u16;
        let len_bytes = msg_len.to_be_bytes();

        // Write length + message atomically
        stream.write_all(&len_bytes).await?;
        stream.write_all(&msg_bytes).await?;
        stream.flush().await?;

        Ok(())
    }
}
