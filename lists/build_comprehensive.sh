#!/bin/bash
# Build comprehensive blocklists from multiple quality sources

echo "Building comprehensive Islamic content blocklists..."

# Keep our hand-curated entries
grep -E "^0\.0\.0\.0/0 -\| " adult_original.txt | sed 's/0\.0\.0\.0\/0 -| //' | sed 's/\*\.//' > adult_curated.txt
grep -E "^0\.0\.0\.0/0 -\| " gambling_original.txt | sed 's/0\.0\.0\.0\/0 -| //' | sed 's/\*\.//' > gambling_curated.txt

# Download fresh quality lists
echo "Downloading blocklistproject adult list..."
curl -s "https://raw.githubusercontent.com/blocklistproject/Lists/master/porn.txt" | \
    grep "^0.0.0.0" | awk '{print $2}' | sort -u > /tmp/adult_block.txt

echo "Downloading blocklistproject gambling list..."
curl -s "https://raw.githubusercontent.com/blocklistproject/Lists/master/gambling.txt" | \
    grep "^0.0.0.0" | awk '{print $2}' | sort -u > /tmp/gambling_block.txt

# Merge and dedupe (keep top sites only for performance)
cat adult_curated.txt /tmp/adult_block.txt | sort -u | head -2500 > adult_merged.txt
cat gambling_curated.txt /tmp/gambling_block.txt | sort -u | head -1500 > gambling_merged.txt

# Convert to noorDNS format
echo "# Adult content blocklist - Comprehensive" > adult.txt
echo "# Sources: Hand-curated + BlockListProject" >> adult.txt
echo "# Total domains: $(wc -l < adult_merged.txt)" >> adult.txt
echo "" >> adult.txt
echo "# Wildcard TLDs" >> adult.txt
echo "0.0.0.0/0 -| *.porn" >> adult.txt
echo "0.0.0.0/0 -| *.xxx" >> adult.txt
echo "0.0.0.0/0 -| *.adult" >> adult.txt
echo "0.0.0.0/0 -| *.sex" >> adult.txt
echo "" >> adult.txt
cat adult_merged.txt | while read domain; do
    echo "0.0.0.0/0 -| $domain"
    echo "0.0.0.0/0 -| *.$domain"
done >> adult.txt

echo "# Gambling sites blocklist - Comprehensive" > gambling.txt
echo "# Sources: Hand-curated + BlockListProject" >> gambling.txt  
echo "# Total domains: $(wc -l < gambling_merged.txt)" >> gambling.txt
echo "" >> gambling.txt
echo "# Wildcard TLDs" >> gambling.txt
echo "0.0.0.0/0 -| *.casino" >> gambling.txt
echo "0.0.0.0/0 -| *.bet" >> gambling.txt
echo "0.0.0.0/0 -| *.poker" >> gambling.txt
echo "0.0.0.0/0 -| *.slots" >> gambling.txt
echo "0.0.0.0/0 -| *.gambling" >> gambling.txt
echo "" >> gambling.txt
cat gambling_merged.txt | while read domain; do
    echo "0.0.0.0/0 -| $domain"
    echo "0.0.0.0/0 -| *.$domain"
done >> gambling.txt

echo "Done! Created comprehensive blocklists:"
wc -l adult.txt gambling.txt

# Cleanup
rm -f *_merged.txt *_curated.txt /tmp/adult_block.txt /tmp/gambling_block.txt
