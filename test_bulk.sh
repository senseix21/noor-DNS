#!/usr/bin/env bash

# noorDNS Testing Script
# Tests Islamic content filtering functionality

if [ $# -ne 2 ]; then
    echo "Usage: $0 <DNS_IP> <DNS_PORT>"
    echo "Example: $0 127.0.0.1 8053"
    exit 1
fi

DNS_IP=$1
DNS_PORT=$2

echo "🧪 Testing noorDNS Islamic Content Filtering"
echo "DNS Server: $DNS_IP:$DNS_PORT"
echo "==========================================="

# Test function
test_domain() {
    local domain=$1
    local expected=$2
    echo -n "Testing $domain... "
    
    result=$(dig @$DNS_IP -p $DNS_PORT "$domain" +short +time=3 2>/dev/null)
    
    if [ -z "$result" ]; then
        if [ "$expected" = "BLOCKED" ]; then
            echo "✅ BLOCKED (as expected)"
        else
            echo "❌ BLOCKED (should be allowed)"
        fi
    else
        if [ "$expected" = "ALLOWED" ]; then
            echo "✅ ALLOWED: $result"
        else
            echo "❌ ALLOWED (should be blocked): $result"
        fi
    fi
}

echo "\n🌟 Testing ALLOWED domains (should work):"
echo "----------------------------------------"
test_domain "google.com" "ALLOWED"
test_domain "github.com" "ALLOWED"
test_domain "microsoft.com" "ALLOWED"
test_domain "apple.com" "ALLOWED"
test_domain "amazon.com" "ALLOWED"
test_domain "wikipedia.org" "ALLOWED"
test_domain "stackoverflow.com" "ALLOWED"
test_domain "bbc.com" "ALLOWED"
test_domain "cnn.com" "ALLOWED"
test_domain "cloudflare.com" "ALLOWED"

echo "\n🛡️ Testing BLOCKED domains (adult content):"
echo "--------------------------------------------"
test_domain "pornhub.com" "BLOCKED"
test_domain "xvideos.com" "BLOCKED"
test_domain "xnxx.com" "BLOCKED"
test_domain "redtube.com" "BLOCKED"
test_domain "youporn.com" "BLOCKED"
test_domain "onlyfans.com" "BLOCKED"
test_domain "chaturbate.com" "BLOCKED"
test_domain "livejasmin.com" "BLOCKED"

echo "\n🎰 Testing BLOCKED domains (gambling):"
echo "--------------------------------------"
test_domain "bet365.com" "BLOCKED"
test_domain "pokerstars.com" "BLOCKED"

echo "\n🏁 Testing complete!"
echo "=================="
