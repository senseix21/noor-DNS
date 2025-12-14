// Simple self-signed cert generation for DoT
// No BS, just works

use anyhow::{Context, Result};
use rcgen::{CertificateParams, KeyPair};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::sync::Arc;

pub struct TlsConfig {
    pub cert: Vec<CertificateDer<'static>>,
    pub key: PrivateKeyDer<'static>,
}

impl TlsConfig {
    // Generate self-signed cert - simple, works
    pub fn generate_self_signed() -> Result<Self> {
        let mut params = CertificateParams::new(vec!["localhost".to_string()])
            .context("Failed to create cert params")?;
        
        params.distinguished_name.push(
            rcgen::DnType::CommonName,
            "noorDNS DoT Server"
        );

        let key_pair = KeyPair::generate()?;
        let cert = params.self_signed(&key_pair)
            .context("Failed to generate self-signed cert")?;

        let cert_der = CertificateDer::from(cert.der().to_vec());
        let key_der = PrivateKeyDer::try_from(key_pair.serialize_der())
            .map_err(|e| anyhow::anyhow!("Failed to serialize private key: {}", e))?;

        Ok(TlsConfig {
            cert: vec![cert_der],
            key: key_der,
        })
    }

    // Build rustls server config
    pub fn server_config(self) -> Result<Arc<rustls::ServerConfig>> {
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(self.cert, self.key)
            .context("Failed to build TLS server config")?;

        Ok(Arc::new(config))
    }
}
