#!/usr/bin/env bash

DOMAINS=(
example.com
google.com
github.com
microsoft.com
apple.com
amazon.com
facebook.com
twitter.com
linkedin.com
youtube.com
wikipedia.org
yahoo.com
reddit.com
netflix.com
instagram.com
whatsapp.com
pinterest.com
bing.com
live.com
office.com
stackoverflow.com
quora.com
medium.com
dropbox.com
adobe.com
paypal.com
ebay.com
cnn.com
bbc.com
nytimes.com
espn.com
hulu.com
spotify.com
zoom.us
slack.com
twitch.tv
salesforce.com
cloudflare.com
mozilla.org
wordpress.com
duckduckgo.com
)

for domain in "${DOMAINS[@]}"; do
    echo "Querying $domain ..."
    dig @$1 -p $2 "$domain" +short
    echo "-------------------------------------"
done