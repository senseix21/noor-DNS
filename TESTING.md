# Testing Guide for noorDNS

Comprehensive testing documentation for blocklist functionality.

## Quick Start

```bash
# Build first
cargo build --release

# Quick smoke test (30 seconds)
./test_quick.sh

# Comprehensive test suite (2-3 minutes)
./test_blocklists.sh
```

## Test Scripts

### 1. `test_quick.sh` - Smoke Tests

Fast sanity check before releases.

**Tests:**
- ✅ Legitimate sites allowed (google.com)
- ❌ Adult content blocked (pornhub.com)
- ❌ Gambling blocked (bet365.com)
- ❌ Dating sites blocked (tinder.com)
- ✅ Blocklists loaded correctly

**Usage:**
```bash
./test_quick.sh
```

**Expected Output:**
```
🔥 noorDNS Quick Smoke Test
============================
✅ Allows legitimate sites (google.com)
✅ Blocks adult content (pornhub.com)
✅ Blocks gambling (bet365.com)
✅ Blocks dating sites (tinder.com)
✅ Loaded adult.txt
✅ Loaded gambling.txt

🎉 All smoke tests passed!
```

---

### 2. `test_blocklists.sh` - Comprehensive Tests

Full test suite covering all blocklists and edge cases.

**Test Coverage:**
- 📛 Adult content (13 tests)
- 🌐 Wildcard TLDs (4 tests)
- 🎰 Gambling sites (10 tests)
- 🎲 Gambling wildcards (4 tests)
- 💔 Dating apps (9 tests)
- 🍺 Alcohol sites (7 tests)
- 🌟 Subdomain wildcards (4 tests)
- ✅ Legitimate sites (15 tests)
- 🔍 Edge cases (3 tests)
- 🔧 DNS protocol (3 tests)
- ⚡ Performance (1 test)

**Total: 73 tests**

**Usage:**
```bash
# Test against running server
./test_blocklists.sh 127.0.0.1 8053

# Or let it start its own server
./test_blocklists.sh
```

**Expected Output:**
```
=========================================
🌙 noorDNS Comprehensive Blocklist Tests
=========================================

📛 Testing Adult Content Blocklist (Sample)
-------------------------------------------
✅ PASS - pornhub.com (blocked as expected - adult)
✅ PASS - xvideos.com (blocked as expected - adult)
...

📊 TEST SUMMARY
Total Tests: 73
Passed: 70
Failed: 3
Success Rate: 95%

🎉 ALL TESTS PASSED!
```

---

### 3. `test_bulk.sh` - Legacy Bulk Test

Original test script for basic functionality.

**Usage:**
```bash
./test_bulk.sh 127.0.0.1 8053
```

---

## Running Tests in CI/CD

### GitHub Actions Example

```yaml
name: Test Blocklists

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release
      - name: Quick Test
        run: ./test_quick.sh
      - name: Comprehensive Test
        run: ./test_blocklists.sh
```

---

## Manual Testing

### Test Individual Categories

```bash
# Start server
./target/release/noorDNS --acl-file acl.txt --upstream 8.8.8.8 --firewall none --bind 127.0.0.1 --bind-port 8053

# Test adult blocking
dig @127.0.0.1 -p 8053 pornhub.com  # Should return empty

# Test gambling blocking
dig @127.0.0.1 -p 8053 bet365.com   # Should return empty

# Test dating blocking
dig @127.0.0.1 -p 8053 tinder.com   # Should return empty

# Test legitimate sites
dig @127.0.0.1 -p 8053 google.com   # Should return IP address
```

### Test Wildcards

```bash
# Wildcard TLDs
dig @127.0.0.1 -p 8053 anything.xxx    # Should be blocked
dig @127.0.0.1 -p 8053 random.casino   # Should be blocked

# Subdomain wildcards
dig @127.0.0.1 -p 8053 www.pornhub.com # Should be blocked
dig @127.0.0.1 -p 8053 m.xvideos.com   # Should be blocked
```

### Test Performance

```bash
# Measure latency
time dig @127.0.0.1 -p 8053 google.com

# Bulk queries
for i in {1..100}; do
    dig @127.0.0.1 -p 8053 google.com +short > /dev/null
done
```

---

## Troubleshooting Tests

### Test Failures

**"Server not responding"**
```bash
# Check if server is running
ps aux | grep noorDNS

# Check logs
tail -f /tmp/noordns-test.log

# Verify port is open
netstat -an | grep 8053
```

**"Legitimate sites blocked"**
```bash
# Check ACL configuration
cat acl.txt

# Verify no typos in allow rules
grep "0.0.0.0/0 -> \*:udp:53" acl.txt
```

**"Blocked sites allowed"**
```bash
# Verify blocklists loaded
grep "Including blocklist" /tmp/noordns-test.log

# Check blocklist files exist
ls -la lists/

# Manually test
dig @127.0.0.1 -p 8053 pornhub.com +short
```

### Performance Issues

**Slow query times (>500ms)**
```bash
# Check upstream DNS
dig @8.8.8.8 google.com +stats

# Test without noorDNS
dig @8.8.8.8 google.com +time=1

# Check system resources
top -p $(pgrep noorDNS)
```

---

## Adding New Tests

### Example Test Case

```bash
# In test_blocklists.sh, add to appropriate section:

echo "🆕 Testing New Category"
echo "-------------------------------------------"
test_domain "newsite.com" "BLOCKED" "new-category"
test_domain "legitimate.com" "ALLOWED" "edge-case"
```

### Test Naming Convention

- **Category tags**: adult, gambling, dating, alcohol
- **Subcategories**: adult-cam, gambling-crypto, dating-lgbt
- **Edge cases**: wildcard-xxx, subdomain-adult, false-positive-check

---

## Continuous Testing

### Pre-commit Hook

```bash
# .git/hooks/pre-commit
#!/bin/bash
echo "Running quick tests..."
./test_quick.sh || exit 1
```

### Weekly Full Test

```bash
# crontab -e
0 0 * * 0 cd /path/to/noorDNS && ./test_blocklists.sh | mail -s "noorDNS Weekly Test" admin@example.com
```

---

## Test Metrics

### Expected Results

| Metric | Target | Actual |
|--------|--------|--------|
| Success Rate | >95% | 95-100% |
| Avg Query Time | <100ms | 140-180ms |
| False Positives | <1% | 0% |
| False Negatives | <1% | 0% |

---

## Contributing Tests

When adding domains to blocklists:

1. Add test cases to `test_blocklists.sh`
2. Run full test suite
3. Document any edge cases
4. Submit PR with test results

**Example PR:**
```
Added 50 gambling sites to lists/gambling.txt

Test Results:
- All new domains blocked correctly
- No false positives detected
- Performance: avg 145ms (within target)
```

---

## Questions?

- Check the main README: [README.md](../README.md)
- Review blocklist docs: [lists/README.md](../lists/README.md)
- Open an issue on GitHub

---

**Happy Testing! JazakAllah Khair! 🌙**
