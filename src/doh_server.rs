// DNS-over-HTTPS (DoH) server implementation
// RFC 8484 - simple, no BS

use hickory_proto::op::Message;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use base64::Engine;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::proxy_server::message_processor::DnsMessageProcessor;

const DNS_CONTENT_TYPE: &str = "application/dns-message";
const MAX_DNS_SIZE: usize = 65535;

type DohResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct DohServer {
    processor: Arc<DnsMessageProcessor>,
}

impl DohServer {
    pub fn new(processor: Arc<DnsMessageProcessor>) -> Self {
        Self { processor }
    }

    pub async fn run(
        self,
        bind_addr: SocketAddr,
        tls_acceptor: TlsAcceptor,
    ) -> DohResult<()> {
        let listener = TcpListener::bind(bind_addr).await?;
        log::info!("DoH server listening on https://{}", bind_addr);

        let server = Arc::new(self);

        loop {
            let (stream, peer) = listener.accept().await?;
            let tls_stream = match tls_acceptor.accept(stream).await {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("TLS handshake failed from {}: {}", peer, e);
                    continue;
                }
            };

            let server = Arc::clone(&server);
            tokio::spawn(async move {
                let io = TokioIo::new(tls_stream);
                let service = service_fn(move |req| {
                    let server = Arc::clone(&server);
                    async move { server.handle_request(req, peer).await }
                });

                if let Err(e) = http2::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service)
                    .await
                {
                    log::debug!("HTTP/2 connection error: {}", e);
                }
            });
        }
    }

    async fn handle_request(
        &self,
        req: Request<hyper::body::Incoming>,
        peer: SocketAddr,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        if req.uri().path() != "/dns-query" {
            return Ok(error_response(StatusCode::NOT_FOUND));
        }

        match req.method() {
            &Method::POST => self.handle_post(req, peer).await,
            &Method::GET => self.handle_get(req, peer).await,
            _ => Ok(error_response(StatusCode::METHOD_NOT_ALLOWED)),
        }
    }

    async fn handle_post(
        &self,
        req: Request<hyper::body::Incoming>,
        peer: SocketAddr,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        if !check_content_type(&req) {
            return Ok(error_response(StatusCode::UNSUPPORTED_MEDIA_TYPE));
        }

        let body = match req.into_body().collect().await {
            Ok(b) => b.to_bytes(),
            Err(_) => return Ok(error_response(StatusCode::BAD_REQUEST)),
        };

        if body.len() > MAX_DNS_SIZE {
            return Ok(error_response(StatusCode::PAYLOAD_TOO_LARGE));
        }

        self.process_dns(&body, peer).await
    }

    async fn handle_get(
        &self,
        req: Request<hyper::body::Incoming>,
        peer: SocketAddr,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        let query = req.uri().query().unwrap_or("");
        let dns_param = query
            .split('&')
            .find(|p| p.starts_with("dns="))
            .and_then(|p| p.strip_prefix("dns="));

        let dns_param = match dns_param {
            Some(p) => p,
            None => return Ok(error_response(StatusCode::BAD_REQUEST)),
        };

        let decoded = match base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(dns_param) {
            Ok(d) => d,
            Err(_) => return Ok(error_response(StatusCode::BAD_REQUEST)),
        };

        if decoded.len() > MAX_DNS_SIZE {
            return Ok(error_response(StatusCode::PAYLOAD_TOO_LARGE));
        }

        self.process_dns(&decoded, peer).await
    }

    async fn process_dns(
        &self,
        query_bytes: &[u8],
        peer: SocketAddr,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        let mut buffer = query_bytes.to_vec();
        
        let reaction = self.processor.process_client_request(peer.ip(), &mut buffer);
        
        let response_bytes = match reaction {
            crate::proxy_server::message_processor::RequestReaction::Discard => {
                return Ok(error_response(StatusCode::BAD_REQUEST));
            }
            crate::proxy_server::message_processor::RequestReaction::RespondToClient => buffer,
            crate::proxy_server::message_processor::RequestReaction::ForwardToUpstream { .. } => {
                // For simplicity, return REFUSED for now (upstream forwarding comes later)
                let query = Message::from_vec(query_bytes).unwrap();
                let mut response = Message::new();
                response.set_id(query.id());
                response.set_response_code(hickory_proto::op::ResponseCode::Refused);
                response.to_vec().unwrap()
            }
        };

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", DNS_CONTENT_TYPE)
            .header("content-length", response_bytes.len())
            .body(Full::new(Bytes::from(response_bytes)))
            .unwrap())
    }
}

fn check_content_type(req: &Request<hyper::body::Incoming>) -> bool {
    req.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == DNS_CONTENT_TYPE)
        .unwrap_or(false)
}

fn error_response(status: StatusCode) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(Bytes::new()))
        .unwrap()
}
