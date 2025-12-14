#!/usr/bin/env bash
# Quick smoke test - runs fast, tests basics

set -e

cd "$(dirname "$0")"

echo "🔥 noorDNS Quick Smoke Test"
echo "============================"

# Start server
echo "Starting server..."
./target/debug/noorDNS --acl-file acl.txt --upstream 8.8.8.8 --firewall none --bind 127.0.0.1 --bind-port 8053 > /tmp/noordns-test.log 2>&1 &
SERVER_PID=$!
sleep 2

cleanup() {
    kill $SERVER_PID 2>/dev/null || true
}
trap cleanup EXIT

# Quick tests
echo "Testing..."

# Should allow
if dig @127.0.0.1 -p 8053 google.com +short +time=1 2>&1 | grep -qE '^[0-9]+\.[0-9]+'; then
    echo "✅ Allows legitimate sites (google.com)"
else
    echo "❌ Failed to allow google.com"
    exit 1
fi

# Should block adult
if dig @127.0.0.1 -p 8053 pornhub.com +short +time=1 2>&1 | grep -q "query response not set\|^$"; then
    echo "✅ Blocks adult content (pornhub.com)"
else
    echo "❌ Failed to block pornhub.com"
    exit 1
fi

# Should block gambling
if dig @127.0.0.1 -p 8053 bet365.com +short +time=1 2>&1 | grep -q "query response not set\|^$"; then
    echo "✅ Blocks gambling (bet365.com)"
else
    echo "❌ Failed to block bet365.com"
    exit 1
fi

# Should block dating
if dig @127.0.0.1 -p 8053 tinder.com +short +time=1 2>&1 | grep -q "query response not set\|^$"; then
    echo "✅ Blocks dating sites (tinder.com)"
else
    echo "❌ Failed to block tinder.com"
    exit 1
fi

# Check logs loaded blocklists
if grep -q "Including blocklist: lists/adult.txt" /tmp/noordns-test.log; then
    echo "✅ Loaded adult.txt"
else
    echo "❌ Did not load adult.txt"
    exit 1
fi

if grep -q "Including blocklist: lists/gambling.txt" /tmp/noordns-test.log; then
    echo "✅ Loaded gambling.txt"
else
    echo "❌ Did not load gambling.txt"
    exit 1
fi

echo ""
echo "🎉 All smoke tests passed!"
echo "Run ./test_blocklists.sh for comprehensive tests"
