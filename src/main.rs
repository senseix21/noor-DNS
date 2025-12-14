mod access_control_list_parser;
mod access_control_tree;
mod dot_server;
mod dot_client;mod firewall_backend;
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
use std::sync::Arc;
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
        Some((
            DotServer::new(dot_addr).await.context("Failed to start DoT server")?,
            proxy_server.clone(),
            options.proxy_server.upstream,
            options.proxy_server.upstream_port,
        ))
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
        if let Some((dot, proxy, upstream_ip, upstream_port)) = dot_server {
            run_dot_server(dot, proxy, upstream_ip, upstream_port).await
        } else {
            std::future::pending().await
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
async fn run_dot_server(
    server: DotServer,
    proxy: Arc<ProxyServer>,
    upstream_ip: std::net::IpAddr,
    upstream_port: u16,
) -> anyhow::Result<()> {
    use crate::dot_client::DotClient;
    
    let upstream_addr = SocketAddr::new(upstream_ip, upstream_port);
    let dot_client = DotClient::new(upstream_addr)?;
    
    loop {
        match server.accept().await {
            Ok((mut stream, addr)) => {
                let proxy_clone = proxy.clone();
                let client = dot_client.clone();
                
                tokio::spawn(async move {
                    let timeout = tokio::time::Duration::from_secs(10);
                    match tokio::time::timeout(timeout, handle_dot_connection(&mut stream, addr, proxy_clone, client)).await {
                        Ok(Ok(())) => log::debug!("DoT connection from {} completed", addr),
                        Ok(Err(e)) => log::warn!("DoT connection error from {}: {}", addr, e),
                        Err(_) => log::warn!("DoT connection from {} timed out", addr),
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
    proxy: Arc<ProxyServer>,
    dot_client: crate::dot_client::DotClient,
) -> anyhow::Result<()> {
    use crate::proxy_server::message_processor::RequestReaction;
    
    // Read DNS query
    let query = DotServer::read_message(stream).await?;
    log::info!("DoT query from {}: {:?}", addr, query.queries());
    
    // Serialize query for message processor
    let mut buffer = query.to_vec()?;
    
    // Process through ACL
    let reaction = proxy.message_processor().process_client_request(addr.ip(), &mut buffer);
    
    match reaction {
        RequestReaction::Discard => {
            log::warn!("DoT query blocked by ACL from {}", addr);
            return Ok(());
        }
        RequestReaction::RespondToClient => {
            // Blocked domain, send response from buffer
            let response = hickory_proto::op::Message::from_vec(&buffer)?;
            DotServer::write_message(stream, &response).await?;
        }
        RequestReaction::ForwardToUpstream { forwarded_request } => {
            // Forward to upstream DoT server
            let query_msg = hickory_proto::op::Message::from_vec(&buffer)?;
            
            match dot_client.query(&query_msg).await {
                Ok(mut response) => {
                    // Process response through message processor
                    let mut response_buf = response.to_vec()?;
                    
                    let response_reaction = proxy.message_processor()
                        .process_upstream_response(addr.ip(), &mut response_buf, &forwarded_request)
                        .await;
                    
                    use crate::proxy_server::message_processor::ResponseReaction;
                    match response_reaction {
                        ResponseReaction::ForwardToClient => {
                            let final_response = hickory_proto::op::Message::from_vec(&response_buf)?;
                            DotServer::write_message(stream, &final_response).await?;
                        }
                        ResponseReaction::Discard => {
                            log::debug!("DoT response discarded by processor");
                        }
                    }
                }
                Err(e) => {
                    log::error!("Upstream DoT query failed: {}", e);
                    return Err(e);
                }
            }
        }
    }
    
    Ok(())
}
