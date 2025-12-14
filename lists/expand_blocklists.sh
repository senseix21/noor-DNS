#!/bin/bash
# Expand blocklists to 5000+ domains
# Linus style: simple, direct, gets the job done

set -e

LISTS_DIR="$(cd "$(dirname "$0")" && pwd)"
TMP_DIR="/tmp/noordns_blocklist_$$"

mkdir -p "$TMP_DIR"
trap "rm -rf $TMP_DIR" EXIT

echo "🌙 noorDNS Blocklist Expansion"
echo "=============================="
echo ""

# Adult content expansion
echo "📥 Downloading adult content blocklists..."
{
    curl -fsSL "https://raw.githubusercontent.com/blocklistproject/Lists/master/porn.txt" | \
        grep -E "^0\.0\.0\.0" | awk '{print $2}' || true
    
    curl -fsSL "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/porn/hosts" | \
        grep -E "^0\.0\.0\.0" | awk '{print $2}' | grep -v "0.0.0.0" || true
} | sort -u > "$TMP_DIR/adult_extra.txt"

ADULT_COUNT=$(wc -l < "$TMP_DIR/adult_extra.txt")
echo "   Found $ADULT_COUNT additional adult domains"

# Gambling expansion
echo "📥 Downloading gambling blocklists..."
{
    curl -fsSL "https://raw.githubusercontent.com/blocklistproject/Lists/master/gambling.txt" | \
        grep -E "^0\.0\.0\.0" | awk '{print $2}' || true
    
    curl -fsSL "https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling/hosts" | \
        grep -E "^0\.0\.0\.0" | awk '{print $2}' | grep -v "0.0.0.0" || true
} | sort -u > "$TMP_DIR/gambling_extra.txt"

GAMBLING_COUNT=$(wc -l < "$TMP_DIR/gambling_extra.txt")
echo "   Found $GAMBLING_COUNT additional gambling domains"

# Dating apps expansion
echo "📥 Downloading dating blocklists..."
{
    curl -fsSL "https://raw.githubusercontent.com/blocklistproject/Lists/master/dating.txt" | \
        grep -E "^0\.0\.0\.0" | awk '{print $2}' || true
} | sort -u > "$TMP_DIR/dating_extra.txt"

DATING_COUNT=$(wc -l < "$TMP_DIR/dating_extra.txt")
echo "   Found $DATING_COUNT additional dating domains"

# Alcohol expansion (use fraud/scam lists as proxy)
echo "📥 Creating alcohol blocklist..."
cat > "$TMP_DIR/alcohol_extra.txt" << 'EOF'
drizly.com
wine.com
totalwine.com
reservebar.com
craftshack.com
caskers.com
winc.com
nakedwines.com
wine-searcher.com
vinepair.com
bevmo.com
saucey.com
minibar.com
wineaccess.com
klwines.com
wine.net
wine-club.com
winelibrary.com
zachys.com
winebid.com
EOF

ALCOHOL_COUNT=$(wc -l < "$TMP_DIR/alcohol_extra.txt")
echo "   Found $ALCOHOL_COUNT additional alcohol domains"

echo ""
echo "🔧 Merging with existing lists..."

# Backup originals
cp "$LISTS_DIR/adult.txt" "$LISTS_DIR/adult_backup_$(date +%Y%m%d).txt" 2>/dev/null || true
cp "$LISTS_DIR/gambling.txt" "$LISTS_DIR/gambling_backup_$(date +%Y%m%d).txt" 2>/dev/null || true
cp "$LISTS_DIR/dating.txt" "$LISTS_DIR/dating_backup_$(date +%Y%m%d).txt" 2>/dev/null || true
cp "$LISTS_DIR/alcohol.txt" "$LISTS_DIR/alcohol_backup_$(date +%Y%m%d).txt" 2>/dev/null || true

# Extract existing domains (skip ACL syntax)
grep -E "^[0-9.]+ -\|" "$LISTS_DIR/adult.txt" | awk '{print $3}' > "$TMP_DIR/adult_current.txt" || true
grep -E "^[0-9.]+ -\|" "$LISTS_DIR/gambling.txt" | awk '{print $3}' > "$TMP_DIR/gambling_current.txt" || true
grep -E "^[0-9.]+ -\|" "$LISTS_DIR/dating.txt" | awk '{print $3}' > "$TMP_DIR/dating_current.txt" || true
grep -E "^[0-9.]+ -\|" "$LISTS_DIR/alcohol.txt" | awk '{print $3}' > "$TMP_DIR/alcohol_current.txt" || true

# Merge and deduplicate
cat "$TMP_DIR/adult_current.txt" "$TMP_DIR/adult_extra.txt" | \
    grep -v "^$" | sort -u > "$TMP_DIR/adult_merged.txt"

cat "$TMP_DIR/gambling_current.txt" "$TMP_DIR/gambling_extra.txt" | \
    grep -v "^$" | sort -u > "$TMP_DIR/gambling_merged.txt"

cat "$TMP_DIR/dating_current.txt" "$TMP_DIR/dating_extra.txt" | \
    grep -v "^$" | sort -u > "$TMP_DIR/dating_merged.txt"

cat "$TMP_DIR/alcohol_current.txt" "$TMP_DIR/alcohol_extra.txt" | \
    grep -v "^$" | sort -u > "$TMP_DIR/alcohol_merged.txt"

# Generate new lists with ACL syntax
echo "# Adult content blocklist - Auto-generated" > "$LISTS_DIR/adult.txt"
echo "# $(date)" >> "$LISTS_DIR/adult.txt"
echo "" >> "$LISTS_DIR/adult.txt"
while IFS= read -r domain; do
    echo "0.0.0.0/0 -| $domain" >> "$LISTS_DIR/adult.txt"
    echo "0.0.0.0/0 -| *.$domain" >> "$LISTS_DIR/adult.txt"
done < "$TMP_DIR/adult_merged.txt"

echo "# Gambling blocklist - Auto-generated" > "$LISTS_DIR/gambling.txt"
echo "# $(date)" >> "$LISTS_DIR/gambling.txt"
echo "" >> "$LISTS_DIR/gambling.txt"
while IFS= read -r domain; do
    echo "0.0.0.0/0 -| $domain" >> "$LISTS_DIR/gambling.txt"
    echo "0.0.0.0/0 -| *.$domain" >> "$LISTS_DIR/gambling.txt"
done < "$TMP_DIR/gambling_merged.txt"

echo "# Dating apps blocklist - Auto-generated" > "$LISTS_DIR/dating.txt"
echo "# $(date)" >> "$LISTS_DIR/dating.txt"
echo "" >> "$LISTS_DIR/dating.txt"
while IFS= read -r domain; do
    echo "0.0.0.0/0 -| $domain" >> "$LISTS_DIR/dating.txt"
    echo "0.0.0.0/0 -| *.$domain" >> "$LISTS_DIR/dating.txt"
done < "$TMP_DIR/dating_merged.txt"

echo "# Alcohol delivery blocklist - Auto-generated" > "$LISTS_DIR/alcohol.txt"
echo "# $(date)" >> "$LISTS_DIR/alcohol.txt"
echo "" >> "$LISTS_DIR/alcohol.txt"
while IFS= read -r domain; do
    echo "0.0.0.0/0 -| $domain" >> "$LISTS_DIR/alcohol.txt"
    echo "0.0.0.0/0 -| *.$domain" >> "$LISTS_DIR/alcohol.txt"
done < "$TMP_DIR/alcohol_merged.txt"

echo ""
echo "✅ Blocklist expansion complete!"
echo ""
echo "📊 Final Statistics:"
echo "   Adult:    $(wc -l < "$LISTS_DIR/adult.txt") rules ($(wc -l < "$TMP_DIR/adult_merged.txt") unique domains)"
echo "   Gambling: $(wc -l < "$LISTS_DIR/gambling.txt") rules ($(wc -l < "$TMP_DIR/gambling_merged.txt") unique domains)"
echo "   Dating:   $(wc -l < "$LISTS_DIR/dating.txt") rules ($(wc -l < "$TMP_DIR/dating_merged.txt") unique domains)"
echo "   Alcohol:  $(wc -l < "$LISTS_DIR/alcohol.txt") rules ($(wc -l < "$TMP_DIR/alcohol_merged.txt") unique domains)"
echo ""

TOTAL_DOMAINS=$(($(wc -l < "$TMP_DIR/adult_merged.txt") + $(wc -l < "$TMP_DIR/gambling_merged.txt") + $(wc -l < "$TMP_DIR/dating_merged.txt") + $(wc -l < "$TMP_DIR/alcohol_merged.txt")))
TOTAL_RULES=$(($(wc -l < "$LISTS_DIR/adult.txt") + $(wc -l < "$LISTS_DIR/gambling.txt") + $(wc -l < "$LISTS_DIR/dating.txt") + $(wc -l < "$LISTS_DIR/alcohol.txt")))

echo "🎯 Total: $TOTAL_DOMAINS unique domains, $TOTAL_RULES total rules"
echo ""
echo "Made with ❤️ for the Ummah 🌙"
