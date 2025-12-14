use assert_matches::assert_matches;
use hickory_proto::xfer::Protocol;
use hickory_resolver::Resolver;
use hickory_resolver::config::{NameServerConfig, ResolverConfig};
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::proto::op::ResponseCode;
use rand::Rng;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::panic::UnwindSafe;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::NamedTempFile;

lazy_static::lazy_static! {
    static ref COMPILED_BINARY_PATH: PathBuf = assert_cmd::cargo::cargo_bin("noorDNS");
}

fn with_server(acl: &str, test: impl FnOnce(u16) + UnwindSafe) {
    let mut acl_file = NamedTempFile::new().expect("Failed to create temp ACL file");
    writeln!(acl_file, "{acl}").unwrap();
    acl_file.flush().unwrap();

    let random_port = rand::rng().random_range(20_000_u16..50_000_u16);

    let mut server = std::process::Command::new(&*COMPILED_BINARY_PATH)
        .args([
            "--acl-file",
            acl_file
                .path()
                .to_str()
                .expect("Temp ACL file path has unrepresentable characters"),
            "--firewall",
            "none",
            "--upstream",
            "8.8.8.8",
            "--bind",
            "127.0.0.1",
            "--bind-port",
            &random_port.to_string(),
        ])
        .spawn()
        .expect("Failed to launch server");

    std::thread::sleep(Duration::from_millis(1000));

    let test_result = std::panic::catch_unwind(|| test(random_port));

    let _ = server.kill();
    server.wait().expect("Failed to join server");

    test_result.expect("Test failed");
}

#[derive(Debug)]
enum ResolveResult {
    Resolved(Vec<IpAddr>),
    Empty(ResponseCode),
    #[expect(unused)]
    Error(hickory_resolver::ResolveError),
}

impl ResolveResult {
    fn assert_refused(self) {
        assert_matches!(self, ResolveResult::Empty(ResponseCode::Refused));
    }

    fn assert_any_ip(self) {
        assert_matches!(self, ResolveResult::Resolved(v) if !v.is_empty());
    }
}

#[must_use]
async fn resolve(server_port: u16, server_protocol: Protocol, domain: &str) -> ResolveResult {
    let resolver_config = ResolverConfig::from_parts(
        None,
        vec![],
        vec![NameServerConfig {
            socket_addr: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), server_port).into(),
            protocol: server_protocol,
            tls_dns_name: None,
            http_endpoint: None,
            trust_negative_responses: true,
            bind_addr: None,
        }],
    );

    let mut resolver_builder =
        Resolver::builder_with_config(resolver_config, TokioConnectionProvider::default());

    let resolver_opts = resolver_builder.options_mut();
    resolver_opts.attempts = 1;
    resolver_opts.cache_size = 0;

    let resolver = resolver_builder.build();

    match resolver.lookup_ip(domain).await {
        Ok(resolved) => ResolveResult::Resolved(resolved.iter().collect()),
        Err(e) => {
            if let hickory_resolver::ResolveErrorKind::Proto(proto_error) = e.kind() {
                if let hickory_proto::ProtoErrorKind::NoRecordsFound { response_code, .. } =
                    proto_error.kind()
                {
                    ResolveResult::Empty(*response_code)
                } else {
                    ResolveResult::Error(e)
                }
            } else {
                ResolveResult::Error(e)
            }
        }
    }
}

#[test]
fn adult_blocklist_blocks_known_sites() {
    let acl = r#"
127.0.0.1 -| pornhub.com
127.0.0.1 -| xvideos.com
127.0.0.1 -| xnxx.com
127.0.0.1 -| onlyfans.com
127.0.0.1 ~> google.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "pornhub.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "xvideos.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "xnxx.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "onlyfans.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();
        })
    })
}

#[test]
fn gambling_blocklist_blocks_known_sites() {
    let acl = r#"
127.0.0.1 -| bet365.com
127.0.0.1 -| betway.com
127.0.0.1 -| pokerstars.com
127.0.0.1 -| draftkings.com
127.0.0.1 ~> google.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "bet365.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "betway.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "pokerstars.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "draftkings.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();
        })
    })
}

#[test]
fn dating_blocklist_blocks_known_sites() {
    let acl = r#"
127.0.0.1 -| tinder.com
127.0.0.1 -| bumble.com
127.0.0.1 -| match.com
127.0.0.1 -| okcupid.com
127.0.0.1 ~> google.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "tinder.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "bumble.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "match.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "okcupid.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();
        })
    })
}

#[test]
fn alcohol_blocklist_blocks_known_sites() {
    let acl = r#"
127.0.0.1 -| drizly.com
127.0.0.1 -| totalwine.com
127.0.0.1 -| wine.com
127.0.0.1 ~> google.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "drizly.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "totalwine.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "wine.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();
        })
    })
}

#[test]
fn wildcard_blocking_works() {
    let acl = r#"
127.0.0.1 -| *.pornhub.com
127.0.0.1 -| *.xvideos.com
127.0.0.1 ~> google.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "www.pornhub.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "m.xvideos.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();
        })
    })
}

#[test]
fn mixed_blocklists_work() {
    let acl = r#"
127.0.0.1 -| pornhub.com
127.0.0.1 -| bet365.com
127.0.0.1 -| tinder.com
127.0.0.1 -| drizly.com
127.0.0.1 ~> google.com
127.0.0.1 ~> github.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "pornhub.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "bet365.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "tinder.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "drizly.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();

            resolve(port, Protocol::Udp, "github.com")
                .await
                .assert_any_ip();
        })
    })
}

#[test]
fn tcp_and_udp_both_block() {
    let acl = r#"
127.0.0.1 -| pornhub.com
127.0.0.1 ~> google.com
"#;

    with_server(acl, |port| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            resolve(port, Protocol::Udp, "pornhub.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Tcp, "pornhub.com")
                .await
                .assert_refused();

            resolve(port, Protocol::Udp, "google.com")
                .await
                .assert_any_ip();

            resolve(port, Protocol::Tcp, "google.com")
                .await
                .assert_any_ip();
        })
    })
}
