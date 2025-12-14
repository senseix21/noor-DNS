mod access_control_list_parser;
mod access_control_tree;
mod dot_server;
mod firewall_backend;
mod program_config;
mod protocol;
mod proxy_server;
mod tls_config;

use crate::dot_server::DotServer;
use crate::firewall_backend::FirewallBackend;
use crate::firewall_backend::iptables::IptablesFirewallBackend;
use crate::firewall_backend::noop::NoopFirewallBackend;
use crate::program_config::{FirewallKind, ProgramConfig};
use crate::proxy_server::ProxyServer;
use anyhow::Context;
use env_logger::Env;
use std::net::SocketAddr;
use tokio::signal::unix::{SignalKind, signal};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Parse options
    let options = ProgramConfig::parse();
    println!("Parsed options: {:?}", options);

    // Set up logging
    env_logger::Builder::from_env(Env::default().default_filter_or(if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    }))
    .format_timestamp(None)
    .format_module_path(false)
    .init();

    run(options).await
}

async fn run(options: ProgramConfig) -> anyhow::Result<()> {
    let access_control_tree = access_control_list_parser::parse_file(&options.acl_file)
        .with_context(|| {
            format!(
                "Failed to parse access control list file at '{}'",
                options.acl_file.display()
            )
        })?;

    let firewall_backend: Box<dyn FirewallBackend> = match options.firewall.backend {
        FirewallKind::none => Box::new(NoopFirewallBackend::new()),
        FirewallKind::iptables => {
            let chain = options
                .firewall
                .chain
                .unwrap_or_default()
                .trim()
                .to_string();

            if chain.is_empty() {
                anyhow::bail!("Firewall chain is empty, please update your config.");
            }

            Box::new(
                IptablesFirewallBackend::new(chain)
                    .context("Failed to initialize iptables firewall backend")?,
            )
        }
    };

    let proxy_server =
        ProxyServer::new(options.proxy_server, access_control_tree, firewall_backend)
            .await
            .context("Failed to start proxy server")?;

    // Start DoT server if enabled
    let dot_server = if options.proxy_server.enable_dot {
        let dot_addr = SocketAddr::new(options.proxy_server.bind, options.proxy_server.dot_port);
        Some(DotServer::new(dot_addr).await.context("Failed to start DoT server")?)
    } else {
        None
    };

    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigquit = signal(SignalKind::quit()).unwrap();

    log::info!(
        "Server started on [{}]:{}!",
        options.proxy_server.bind,
        options.proxy_server.bind_port
    );
    
    if options.proxy_server.enable_dot {
        log::info!("DoT server enabled on port {}", options.proxy_server.dot_port);
    }

    // Run all servers concurrently
    let proxy_run = proxy_server.run();
    let dot_run = async {
        if let Some(dot) = dot_server {
            run_dot_server(dot).await
        } else {
            std::future::pending().await  // Never completes if DoT disabled
        }
    };

    // Run until a fatal error is encountered or one of the specified signals are received
    (tokio::select! {
        r = proxy_run => r,
        r = dot_run => r,
        _ = sigint.recv() => Ok(()),
        _ = sigterm.recv() => Ok(()),
        _ = sigquit.recv() => Ok(()),
    })?;

    log::info!("Server stopped.");

    Ok(())
}

// DoT server event loop - simple and direct
async fn run_dot_server(server: DotServer) -> anyhow::Result<()> {
    loop {
        match server.accept().await {
            Ok((mut stream, addr)) => {
                log::debug!("DoT connection from {}", addr);
                
                // Spawn handler for this connection
                tokio::spawn(async move {
                    if let Err(e) = handle_dot_connection(&mut stream, addr).await {
                        log::warn!("DoT connection error from {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                log::error!("DoT accept error: {}", e);
            }
        }
    }
}

// Handle single DoT connection
async fn handle_dot_connection(
    stream: &mut tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    // Read DNS query
    let query = DotServer::read_message(stream).await?;
    log::info!("DoT query from {}: {:?}", addr, query.queries());
    
    // TODO: Process through existing DNS pipeline
    // For now, just echo back (will implement in next step)
    
    // Send response
    DotServer::write_message(stream, &query).await?;
    
    Ok(())
}
