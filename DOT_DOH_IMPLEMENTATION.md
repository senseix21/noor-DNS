# DNS-over-TLS (DoT) and DNS-over-HTTPS (DoH) Implementation

## Overview

Add encrypted DNS support to noorDNS for enhanced privacy and security.

---

## Current State

**Supported Protocols:**
- ✅ UDP (port 53)
- ✅ TCP (port 53)

**Missing:**
- ❌ DoT (DNS-over-TLS, port 853)
- ❌ DoH (DNS-over-HTTPS, port 443)

---

## Implementation Plan

### Phase 1: DoT (DNS-over-TLS) - Port 853

**Why DoT First:**
- Simpler than DoH (TLS wrapper over DNS protocol)
- RFC 7858 standard
- Direct upgrade from TCP DNS
- Better performance than DoH

**Components Needed:**
1. TLS listener on port 853
2. TLS certificate management
3. Upstream DoT forwarding
4. ACL support for DoT

**Dependencies:**
```toml
tokio-rustls = "0.26"
rustls = "0.23"
rustls-pemfile = "2.0"
webpki-roots = "0.26"
```

**Implementation Steps:**
1. Add Protocol::Tls to protocol.rs
2. Create TLS server in proxy_server
3. Add certificate loading
4. Support self-signed certs for local use
5. Forward to upstream DoT servers (1.1.1.1, 8.8.8.8)

---

### Phase 2: DoH (DNS-over-HTTPS) - Port 443

**Why DoH Second:**
- More complex (HTTP/2 + TLS wrapper)
- RFC 8484 standard
- Better for bypassing restrictive networks
- Looks like regular HTTPS traffic

**Components Needed:**
1. HTTP/2 server on port 443
2. TLS certificate (reuse from DoT)
3. DNS wire format over HTTP POST/GET
4. Upstream DoH forwarding

**Dependencies:**
```toml
hyper = { version = "1.0", features = ["full"] }
http-body-util = "0.1"
```

**Implementation Steps:**
1. Add Protocol::Https to protocol.rs
2. Create HTTP/2 server with TLS
3. Handle /dns-query endpoint
4. Support GET (base64url) and POST (binary)
5. Forward to upstream DoH (cloudflare, google)

---

## Technical Design

### Protocol Enum Extension

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Udp,
    Tcp,
    Tls,    // DoT - port 853
    Https,  // DoH - port 443
}
```

### Config Extension

```rust
pub struct ProxyServerConfig {
    // ... existing fields ...
    
    // DoT/DoH settings
    pub enable_dot: bool,
    pub enable_doh: bool,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
    pub dot_port: u16,  // default 853
    pub doh_port: u16,  // default 443
}
```

### Certificate Handling

**Development Mode:**
- Generate self-signed cert on startup
- Store in ~/.config/noordns/cert.pem

**Production Mode:**
- Use Let's Encrypt cert
- Support cert auto-renewal
- Integration with certbot

---

## Upstream DoT/DoH Servers

### DoT Servers (Port 853)
```
1.1.1.1         # Cloudflare
8.8.8.8         # Google
9.9.9.9         # Quad9
94.140.14.14    # AdGuard
```

### DoH Servers
```
https://cloudflare-dns.com/dns-query       # Cloudflare
https://dns.google/dns-query               # Google
https://dns.quad9.net/dns-query            # Quad9
https://dns.adguard.com/dns-query          # AdGuard
```

---

## ACL Integration

### Syntax Extension

```
# Allow DoT
0.0.0.0/0 -> *:tls:853

# Allow DoH
0.0.0.0/0 -> *:https:443

# Block specific domains over any encrypted protocol
0.0.0.0/0 -| blocked.com
```

---

## Command Line Args

```bash
# Enable DoT
./noorDNS --enable-dot --dot-port 853 \
          --tls-cert cert.pem --tls-key key.pem

# Enable DoH
./noorDNS --enable-doh --doh-port 443 \
          --tls-cert cert.pem --tls-key key.pem

# Enable both
./noorDNS --enable-dot --enable-doh \
          --tls-cert cert.pem --tls-key key.pem

# Use upstream DoT
./noorDNS --upstream-dot 1.1.1.1:853

# Use upstream DoH
./noorDNS --upstream-doh https://cloudflare-dns.com/dns-query
```

---

## Testing Strategy

### Unit Tests
- Protocol parsing
- TLS handshake
- HTTP/2 request parsing
- Certificate validation

### Integration Tests
- DoT query/response
- DoH query/response
- Blocklist over encrypted DNS
- Performance benchmarks

### Manual Testing
```bash
# Test DoT
kdig @127.0.0.1 -p 853 +tls google.com

# Test DoH
curl -H 'content-type: application/dns-message' \
     https://127.0.0.1/dns-query \
     --data-binary @- <<< "$(dig google.com +short)"
```

---

## Performance Targets

| Metric | UDP/TCP | DoT | DoH |
|--------|---------|-----|-----|
| Latency | 10-50ms | 20-80ms | 30-100ms |
| Throughput | 10k qps | 5k qps | 3k qps |
| Memory | 5MB | 10MB | 15MB |

---

## Security Considerations

### Certificate Security
- Strong cipher suites only (TLS 1.3)
- HSTS headers for DoH
- Certificate pinning option
- Automatic cert rotation

### Privacy
- No logging of DNS queries (optional)
- No upstream tracking
- Local caching to reduce upstream queries
- DNSSEC validation

### Attack Mitigation
- Rate limiting per client
- Connection limits
- DDoS protection
- Invalid query handling

---

## Rollout Plan

### Week 1: DoT Foundation
- [ ] Add Protocol::Tls
- [ ] TLS server listener
- [ ] Self-signed cert generation
- [ ] Basic DoT forwarding
- [ ] Unit tests

### Week 2: DoT Production
- [ ] Production cert support
- [ ] ACL integration
- [ ] Performance tuning
- [ ] Integration tests
- [ ] Documentation

### Week 3: DoH Foundation
- [ ] Add Protocol::Https
- [ ] HTTP/2 server
- [ ] /dns-query endpoint
- [ ] DoH wire format
- [ ] Unit tests

### Week 4: DoH Production
- [ ] Upstream DoH forwarding
- [ ] GET/POST support
- [ ] Integration tests
- [ ] Performance tuning
- [ ] Documentation

---

## User Benefits

### Privacy
- Encrypted DNS queries
- ISP can't see DNS lookups
- Bypass DNS hijacking
- Prevent DNS spoofing

### Security
- TLS 1.3 encryption
- Certificate validation
- DNSSEC support
- MITM protection

### Performance
- HTTP/2 multiplexing (DoH)
- Connection pooling
- Keep-alive connections
- Local caching

### Flexibility
- Choose encryption method
- Multiple upstream providers
- Fallback to unencrypted
- Custom certificates

---

## Islamic Benefits

### Enhanced Privacy
- Protect browsing history
- Prevent ISP tracking
- Secure family filtering
- Private Islamic content access

### Censorship Resistance
- Bypass DNS blocking
- Access Islamic sites blocked by ISPs
- Looks like HTTPS traffic (DoH)
- Hard to detect/block

### Family Safety
- Encrypted blocklist enforcement
- Can't be bypassed by changing DNS
- Secure even on public WiFi
- Protect children's privacy

---

## Linus-Style Implementation

```rust
// Simple, direct, no BS
pub struct TlsListener {
    acceptor: TlsAcceptor,
    listener: TcpListener,
}

impl TlsListener {
    pub async fn accept(&self) -> Result<TlsStream<TcpStream>> {
        let (stream, addr) = self.listener.accept().await?;
        Ok(self.acceptor.accept(stream).await?)
    }
}

// Just works™
```

**Principles:**
- No overengineering
- Keep it simple
- Test everything
- Performance matters
- Security first

---

## Dependencies to Add

```toml
[dependencies]
# Existing
hickory-proto = "0.25.2"
hickory-resolver = "0.25.2"
tokio = { version = "1.38", features = ["full"] }

# New for DoT/DoH
tokio-rustls = "0.26"
rustls = "0.23"
rustls-pemfile = "2.0"
webpki-roots = "0.26"
hyper = { version = "1.0", features = ["full"] }
hyper-util = { version = "0.1", features = ["server"] }
http-body-util = "0.1"
base64 = "0.22"
rcgen = "0.13"  # For self-signed certs
```

---

## File Structure

```
src/
├── protocol.rs           # Add Tls, Https
├── proxy_server/
│   ├── dot_server.rs     # DoT implementation
│   ├── doh_server.rs     # DoH implementation
│   ├── tls_config.rs     # TLS cert loading
│   └── upstream_dot.rs   # Upstream DoT client
├── program_config.rs     # Add DoT/DoH settings
└── main.rs              # Wire everything up

tests/
├── dot_integration.rs    # DoT tests
└── doh_integration.rs    # DoH tests
```

---

## Estimated Effort

- **DoT**: 2-3 days (simpler)
- **DoH**: 3-4 days (HTTP/2 complexity)
- **Testing**: 2 days
- **Documentation**: 1 day
- **Total**: ~2 weeks

---

## Success Criteria

✅ DoT server running on port 853
✅ DoH server running on port 443
✅ Blocklists work over encrypted DNS
✅ Performance within targets
✅ 100% test coverage
✅ Production-ready certs
✅ Documentation complete

---

**Status**: 📋 Ready to implement
**Priority**: High (major feature)
**Difficulty**: Medium (well-documented protocols)

**Let's build encrypted DNS for the Ummah!** 🔐🌙
