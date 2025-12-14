# noorDNS Blocklists

Modular blocklists for Islamic content filtering.

## Available Lists

| List | Domains | Rules | Description |
|------|---------|-------|-------------|
| `adult.txt` | ~2,500 | 5,010 | Adult content, pornography, cam sites, hentai |
| `gambling.txt` | ~1,500 | 3,011 | Casinos, betting, poker, crypto gambling |
| `dating.txt` | ~75 | 149 | Dating apps and hookup sites |
| `alcohol.txt` | ~65 | 131 | Alcohol delivery and brewery sites |

**Total: ~4,140 unique domains with 8,300+ blocking rules**

## Usage

Lists are automatically loaded via `acl.txt`:

```bash
# Core Islamic filtering (mandatory)
@include lists/adult.txt
@include lists/gambling.txt

# Optional (uncomment to enable)
#@include lists/dating.txt
#@include lists/alcohol.txt
```

## Custom Blocks

Create `lists/custom.txt` for your own rules:

```bash
# My custom blocks
0.0.0.0/0 -| example.com
0.0.0.0/0 -| *.badsite.com
```

Then include it in `acl.txt`:

```bash
@include lists/custom.txt
```

## Contributing

To add domains to blocklists:

1. Edit the appropriate list file
2. Use format: `0.0.0.0/0 -| domain.com` or `0.0.0.0/0 -| *.domain.com`
3. Test with: `dig @localhost -p 8053 domain.com`
4. Submit PR

## Maintenance

Lists are manually curated to avoid false positives. We prefer quality over quantity.

**Before submitting domains:**
- ✅ Verify the site hosts haram content
- ✅ Check it's not a CDN or legitimate service
- ✅ Test that blocking doesn't break other sites

## Future Lists

Planned for future releases:
- `social-media.txt` - Optional filtering for time management
- `news.txt` - Sectarian/divisive content (regional)
- `streaming.txt` - Platforms with haram content mixed with halal

## Philosophy

We block specific bad sites, not entire categories of technology. For example:
- ❌ Don't block all social media (useful for dawah)
- ✅ Block specific adult/haram content
- ❌ Don't block all gaming (some games are halal)
- ✅ Block gambling/addiction-focused sites

This keeps the filter practical for modern Muslim families.
