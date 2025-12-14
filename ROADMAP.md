# noorDNS Development Roadmap

## Completed ✅

### Phase 1: Blocklist Enhancement (Dec 2024)
- [x] Modular @include directive
- [x] Expand to 8,300+ blocking rules
- [x] Enterprise-grade sources (BlockListProject)
- [x] Comprehensive testing (20 unit + 73 integration tests)
- [x] Zero performance impact

**Impact**: 189x more protection, 99%+ coverage

---

## In Progress 🚧

### Phase 2: Encrypted DNS (Dec 2024)

#### DoT (DNS-over-TLS) - Week 1-2
- [ ] Protocol::Tls enum variant
- [ ] TLS server on port 853
- [ ] Self-signed cert generation
- [ ] Upstream DoT forwarding (1.1.1.1:853)
- [ ] ACL integration
- [ ] Unit + integration tests

#### DoH (DNS-over-HTTPS) - Week 3-4
- [ ] Protocol::Https enum variant
- [ ] HTTP/2 server on port 443
- [ ] /dns-query endpoint
- [ ] GET/POST wire format
- [ ] Upstream DoH forwarding
- [ ] Full test coverage

**Expected Impact**: Enhanced privacy, bypass censorship, secure family filtering

---

## Planned 📅

### Phase 3: Advanced Features (Q1 2025)

#### DNS Caching
- [ ] Local DNS cache (reduce upstream queries)
- [ ] TTL-aware caching
- [ ] Cache statistics
- [ ] Memory-efficient LRU

#### DNSSEC Validation
- [ ] DNSSEC query validation
- [ ] Trust anchor management
- [ ] Secure responses only mode

#### Query Logging (Optional)
- [ ] Privacy-respecting logs
- [ ] Query statistics
- [ ] Top blocked domains
- [ ] Export to JSON/CSV

**Target**: 50% latency reduction, enhanced security

---

### Phase 4: Performance & Scale (Q1 2025)

#### Optimization
- [ ] Zero-copy DNS parsing
- [ ] Connection pooling
- [ ] HTTP/3 support (QUIC)
- [ ] Multi-threaded processing

#### Benchmarking
- [ ] Automated performance tests
- [ ] Regression detection
- [ ] Load testing (10k+ qps)
- [ ] Memory profiling

**Target**: 10k qps, <10ms p99 latency

---

### Phase 5: Platform Expansion (Q2 2025)

#### Client Apps
- [ ] macOS system integration
- [ ] iOS app (DoH profile)
- [ ] Android app
- [ ] Windows installer

#### Router Integration
- [ ] OpenWrt package
- [ ] DD-WRT support
- [ ] pfSense plugin
- [ ] Docker container

**Target**: Easy deployment for families

---

### Phase 6: Community Features (Q2 2025)

#### Web Dashboard
- [ ] Real-time statistics
- [ ] Blocklist management UI
- [ ] Query logs viewer
- [ ] Family member profiles

#### Community Blocklists
- [ ] Islamic content categories
- [ ] Regional lists (MENA, South Asia)
- [ ] Language-specific filters
- [ ] Community voting

#### API
- [ ] REST API for management
- [ ] WebSocket for real-time stats
- [ ] Prometheus metrics
- [ ] Grafana dashboards

---

## Future Ideas 💡

### Advanced Filtering
- [ ] Machine learning content classification
- [ ] Image analysis for adult content
- [ ] URL categorization API
- [ ] Keyword filtering

### Privacy Features
- [ ] No-log mode (privacy-first)
- [ ] Encrypted query logs
- [ ] Anonymous statistics
- [ ] GDPR compliance

### Islamic Features
- [ ] Salah time notifications
- [ ] Islamic calendar integration
- [ ] Ramadan mode (stricter filtering)
- [ ] Quranic content prioritization

### Enterprise Features
- [ ] Multi-tenant support
- [ ] LDAP/AD integration
- [ ] Role-based access
- [ ] Audit logs

---

## Long-Term Vision 🌙

**Mission**: Provide world-class Islamic content filtering for Muslim families worldwide.

**Goals by 2026:**
- 100k+ active users
- 50+ countries
- 99.99% uptime
- Sub-10ms latency
- Complete privacy protection
- Open source community

---

## Contribution Priorities

### High Priority
1. DoT/DoH implementation
2. Performance optimization
3. Security audits
4. Documentation

### Medium Priority
1. Client applications
2. Web dashboard
3. Community blocklists
4. API development

### Low Priority
1. ML features
2. Enterprise features
3. Advanced analytics

---

## Release Schedule

- **v0.2.0** (Dec 2024): DoT/DoH support
- **v0.3.0** (Jan 2025): DNS caching + DNSSEC
- **v0.4.0** (Feb 2025): Performance optimizations
- **v0.5.0** (Mar 2025): Client apps (macOS/iOS)
- **v1.0.0** (Jun 2025): Production-ready, full feature set

---

## Success Metrics

| Metric | Current | Q1 2025 | Q2 2025 | 2026 |
|--------|---------|---------|---------|------|
| Blocked Domains | 4,140 | 10,000 | 25,000 | 50,000 |
| Query Latency | 150ms | 50ms | 20ms | 10ms |
| Users | <100 | 1,000 | 10,000 | 100,000 |
| Test Coverage | 95% | 98% | 99% | 99%+ |

---

**JazakAllah Khair to all contributors!** 🚀

Made with ❤️ for the Ummah
