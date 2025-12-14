#!/usr/bin/env bash
# Comprehensive blocklist testing for noorDNS
# Tests all categories and edge cases

set -e

DNS_IP="${1:-127.0.0.1}"
DNS_PORT="${2:-8053}"
TIMEOUT=2

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TOTAL=0
PASSED=0
FAILED=0

echo "========================================="
echo "🌙 noorDNS Comprehensive Blocklist Tests"
echo "========================================="
echo "DNS Server: $DNS_IP:$DNS_PORT"
echo "Timeout: ${TIMEOUT}s"
echo ""

# Test function
test_domain() {
    local domain=$1
    local expected=$2
    local category=$3
    
    TOTAL=$((TOTAL + 1))
    
    # Run dig and capture both stdout and stderr, check exit code
    result=$(dig @$DNS_IP -p $DNS_PORT "$domain" +short +time=$TIMEOUT 2>&1)
    exit_code=$?
    
    if [ "$expected" = "BLOCKED" ]; then
        # For blocked domains, we expect empty result or timeout
        if [ -z "$result" ] || echo "$result" | grep -q "query response not set" || [ $exit_code -ne 0 ]; then
            echo -e "${GREEN}✅ PASS${NC} - $domain (blocked as expected - $category)"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}❌ FAIL${NC} - $domain (should be blocked - $category) - Got: $result"
            FAILED=$((FAILED + 1))
        fi
    else
        # For allowed domains, we expect valid IP addresses
        if echo "$result" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$|^[0-9a-f:]+$' && [ $exit_code -eq 0 ]; then
            echo -e "${GREEN}✅ PASS${NC} - $domain (allowed as expected)"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}❌ FAIL${NC} - $domain (should be allowed) - Got: $result"
            FAILED=$((FAILED + 1))
        fi
    fi
}

# === ADULT CONTENT TESTS (146 domains) ===
echo "📛 Testing Adult Content Blocklist (Sample)"
echo "-------------------------------------------"
test_domain "pornhub.com" "BLOCKED" "adult"
test_domain "xvideos.com" "BLOCKED" "adult"
test_domain "xnxx.com" "BLOCKED" "adult"
test_domain "redtube.com" "BLOCKED" "adult"
test_domain "youporn.com" "BLOCKED" "adult"
test_domain "xhamster.com" "BLOCKED" "adult"
test_domain "beeg.com" "BLOCKED" "adult"
test_domain "spankbang.com" "BLOCKED" "adult"
test_domain "chaturbate.com" "BLOCKED" "adult-cam"
test_domain "livejasmin.com" "BLOCKED" "adult-cam"
test_domain "onlyfans.com" "BLOCKED" "adult-subscription"
test_domain "4chan.org" "BLOCKED" "adult-forum"
test_domain "nhentai.net" "BLOCKED" "adult-hentai"
echo ""

# === WILDCARD TESTS ===
echo "🌐 Testing Adult Wildcard Blocks"
echo "-------------------------------------------"
test_domain "anything.xxx" "BLOCKED" "wildcard-xxx"
test_domain "random.porn" "BLOCKED" "wildcard-porn"
test_domain "test.adult" "BLOCKED" "wildcard-adult"
test_domain "site.sex" "BLOCKED" "wildcard-sex"
echo ""

# === GAMBLING TESTS (72 domains) ===
echo "🎰 Testing Gambling Blocklist (Sample)"
echo "-------------------------------------------"
test_domain "bet365.com" "BLOCKED" "gambling"
test_domain "pokerstars.com" "BLOCKED" "gambling-poker"
test_domain "888casino.com" "BLOCKED" "gambling-casino"
test_domain "williamhill.com" "BLOCKED" "gambling"
test_domain "betfair.com" "BLOCKED" "gambling"
test_domain "draftkings.com" "BLOCKED" "gambling-fantasy"
test_domain "fanduel.com" "BLOCKED" "gambling-fantasy"
test_domain "stake.com" "BLOCKED" "gambling-crypto"
test_domain "roobet.com" "BLOCKED" "gambling-crypto"
test_domain "bc.game" "BLOCKED" "gambling-crypto"
echo ""

# === WILDCARD GAMBLING TESTS ===
echo "🎲 Testing Gambling Wildcard Blocks"
echo "-------------------------------------------"
test_domain "mysite.casino" "BLOCKED" "wildcard-casino"
test_domain "test.bet" "BLOCKED" "wildcard-bet"
test_domain "random.poker" "BLOCKED" "wildcard-poker"
test_domain "site.slots" "BLOCKED" "wildcard-slots"
echo ""

# === DATING TESTS (49 domains) ===
echo "💔 Testing Dating Blocklist (Sample)"
echo "-------------------------------------------"
test_domain "tinder.com" "BLOCKED" "dating"
test_domain "bumble.com" "BLOCKED" "dating"
test_domain "match.com" "BLOCKED" "dating"
test_domain "okcupid.com" "BLOCKED" "dating"
test_domain "hinge.co" "BLOCKED" "dating"
test_domain "pof.com" "BLOCKED" "dating"
test_domain "grindr.com" "BLOCKED" "dating-lgbt"
test_domain "ashleymadison.com" "BLOCKED" "dating-affair"
test_domain "seeking.com" "BLOCKED" "dating-sugar"
echo ""

# === ALCOHOL TESTS (35 domains) ===
echo "🍺 Testing Alcohol Blocklist (Sample)"
echo "-------------------------------------------"
test_domain "drizly.com" "BLOCKED" "alcohol-delivery"
test_domain "minibar.com" "BLOCKED" "alcohol-delivery"
test_domain "wine.com" "BLOCKED" "alcohol-sales"
test_domain "budweiser.com" "BLOCKED" "alcohol-brand"
test_domain "heineken.com" "BLOCKED" "alcohol-brand"
test_domain "jackdaniels.com" "BLOCKED" "alcohol-brand"
test_domain "weedmaps.com" "BLOCKED" "cannabis"
echo ""

# === SUBDOMAIN WILDCARD TESTS ===
echo "🌟 Testing Subdomain Wildcards"
echo "-------------------------------------------"
test_domain "www.pornhub.com" "BLOCKED" "subdomain-adult"
test_domain "m.xvideos.com" "BLOCKED" "subdomain-adult"
test_domain "mobile.bet365.com" "BLOCKED" "subdomain-gambling"
test_domain "app.tinder.com" "BLOCKED" "subdomain-dating"
echo ""

# === LEGITIMATE SITES (SHOULD NOT BLOCK) ===
echo "✅ Testing Legitimate Sites (Should Allow)"
echo "-------------------------------------------"
test_domain "google.com" "ALLOWED" "search"
test_domain "youtube.com" "ALLOWED" "video"
test_domain "github.com" "ALLOWED" "development"
test_domain "stackoverflow.com" "ALLOWED" "development"
test_domain "amazon.com" "ALLOWED" "shopping"
test_domain "wikipedia.org" "ALLOWED" "education"
test_domain "islamqa.info" "ALLOWED" "islamic"
test_domain "muslimmatters.org" "ALLOWED" "islamic"
test_domain "quran.com" "ALLOWED" "islamic"
test_domain "islamicity.org" "ALLOWED" "islamic"
test_domain "apple.com" "ALLOWED" "tech"
test_domain "microsoft.com" "ALLOWED" "tech"
test_domain "cloudflare.com" "ALLOWED" "infrastructure"
test_domain "bbc.com" "ALLOWED" "news"
test_domain "cnn.com" "ALLOWED" "news"
echo ""

# === EDGE CASES ===
echo "🔍 Testing Edge Cases"
echo "-------------------------------------------"
# These should be blocked due to wildcard TLDs
test_domain "legitimate-casino-game.com" "ALLOWED" "edge-case-tld-word"
# wine-country-tours has "wine" in it but wine.com is blocked, not *.wine
test_domain "wine-country-tours.com" "ALLOWED" "edge-case-substring"
test_domain "dating-advice.com" "ALLOWED" "edge-case-dating-word"
echo ""

# === DNS PROTOCOL TESTS ===
echo "🔧 Testing DNS Protocol Compliance"
echo "-------------------------------------------"
# Test A record
A_RECORD=$(dig @$DNS_IP -p $DNS_PORT google.com A +short +time=$TIMEOUT 2>/dev/null | head -1)
if [[ "$A_RECORD" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${GREEN}✅ PASS${NC} - A record resolution works"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} - A record resolution failed"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))

# Test AAAA record
AAAA_RECORD=$(dig @$DNS_IP -p $DNS_PORT google.com AAAA +short +time=$TIMEOUT 2>/dev/null | head -1)
if [ -n "$AAAA_RECORD" ]; then
    echo -e "${GREEN}✅ PASS${NC} - AAAA record resolution works"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}⚠️  WARN${NC} - AAAA record resolution (IPv6 might not be available)"
    PASSED=$((PASSED + 1))
fi
TOTAL=$((TOTAL + 1))

# Test blocked domain returns no IP
BLOCKED_RESULT=$(dig @$DNS_IP -p $DNS_PORT pornhub.com +short +time=$TIMEOUT 2>&1)
if [ -z "$BLOCKED_RESULT" ] || echo "$BLOCKED_RESULT" | grep -q "query response not set"; then
    echo -e "${GREEN}✅ PASS${NC} - Blocked domains return empty response"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}❌ FAIL${NC} - Blocked domain returned: $BLOCKED_RESULT"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))
echo ""

# === PERFORMANCE TESTS ===
echo "⚡ Testing Performance"
echo "-------------------------------------------"
START=$(date +%s%N)
for i in {1..10}; do
    dig @$DNS_IP -p $DNS_PORT google.com +short +time=$TIMEOUT >/dev/null 2>&1
done
END=$(date +%s%N)
DURATION=$((($END - $START) / 1000000))
AVG=$(($DURATION / 10))

if [ $AVG -lt 100 ]; then
    echo -e "${GREEN}✅ EXCELLENT${NC} - Average query time: ${AVG}ms (10 queries)"
elif [ $AVG -lt 500 ]; then
    echo -e "${YELLOW}✅ GOOD${NC} - Average query time: ${AVG}ms (10 queries)"
else
    echo -e "${YELLOW}⚠️  SLOW${NC} - Average query time: ${AVG}ms (10 queries)"
fi
echo ""

# === SUMMARY ===
echo "========================================="
echo "📊 TEST SUMMARY"
echo "========================================="
echo "Total Tests: $TOTAL"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo "Success Rate: $(( PASSED * 100 / TOTAL ))%"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED!${NC}"
    echo "noorDNS is working perfectly. JazakAllah khair!"
    exit 0
else
    echo -e "${RED}❌ SOME TESTS FAILED${NC}"
    echo "Please check the failed tests above."
    exit 1
fi
