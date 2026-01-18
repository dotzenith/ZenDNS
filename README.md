<h1 align="center"> ━━━━  ❖  ━━━━ </h1>

## ❖ ZenDNS

ZenDNS is an easy-to-use commandline utility to manage DDNS on [Cloudflare](www.cloudflare.com), [Namecheap](https://www.namecheap.com/), [DuckDNS](https://www.duckdns.org/), and [Porkbun](https://porkbun.com/)


---

## ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/zendns/releases/latest/download/zendns-installer.sh | sh
```

#### Brew
```sh
brew install dotzenith/tap/zendns
```

#### Powershell
```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/dotzenith/zendns/releases/latest/download/zendns-installer.ps1 | iex"
```

#### Cargo
```sh
cargo install zendns
```

#### Binaries
Pre-Compiled binaries for linux, mac, and windows are available in [Releases](https://github.com/dotzenith/zendns/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/ZenDNS.git
cd ZenDNS
cargo build --release
./target/release/zendns
```

---

## ❖ Usage

```
Manage Dynamic DNS with serenity

Usage: zendns [OPTIONS] --config <CONFIG>

Options:
  -c, --config <CONFIG>  The json config file to use
  -l, --log <LOGFILE>    Where the output will be logged, uses stdout if not used
  -f, --force            Overrides the check for caching
  -h, --help             Print help
  -V, --version          Print version
```

#### Normal
```sh
zendns --config /path/to/config.json
```

#### Dedicated Logfile
```sh
zendns --config /path/to/config.json --log /path/to/logfile
```

---

## ❖ Configuration

The configuration looks as follows:

```json
{
    "providers": [
      { ... },
      { ... },
      { ... }
    ]
}
```

### ❖ Cloudflare

Create an API token for your zone in [Profile Settings](https://dash.cloudflare.com/profile/api-tokens). The token must have `Zone::DNS::Read` and `Zone::DNS::Edit` permissions.
`Zone Resource` can be set to `Specific Zone` and set to the zone you want to update. If you would like to use this same token for all other zones, please select `All Zones`.
Leave `Client IP Address Filtering` as is, and define how long this token should stay valid for in the `TTL` section.

The configuration for Cloudflare looks as follows:
```json
{
    "type": "cloudflare",
    "key": "your-api-key",
    "zone": "domain.com",
    "hostname": "hostname.domain.com",
    "ttl": 1,
    "proxied": false
}
```
Notes:
- `hostname` can be set to `@` if you want to update `your-website.com` and not a subdomain
- `ttl` is set to 1 for auto, otherwise it should be between `60` and `86400`

### ❖ Namecheap

See [Namecheap's Guide](https://www.namecheap.com/support/knowledgebase/article.aspx/595/11/how-do-i-enable-dynamic-dns-for-a-domain/) on enabling DDNS

The configuration for Namecheap looks as follows:
```json
{
    "type": "namecheap",
    "password": "your-password-key",
    "host": "your-hostname",
    "domain": "your-domain.com"
}
```

### ❖ DuckDNS

Copy the `token` from DuckDNS profile page

The configuration for DuckDNS looks as follows:
```json
{
    "type": "duckdns",
    "token": "your-token",
    "domain": "your-hostname.duckdns.org"
}
```

### ❖ Porkbun

- Generate [API Tokens](https://porkbun.com/account/api) and copy the `API Key` and `Secret Key`
- Enable `API Access` on the domain you'll be updating

The configuration for Porkbun looks as follows:
```json
{
    "type": "porkbun",
    "domain": "your-domain.com",
    "subdomain": "your-subdomain",
    "apikey": "API Key",
    "secretapikey": "Secret Key",
    "ttl": "600"
}
```

### ❖ All Together

All of the providers can be added to the same file, with multiple entries per provider as well

```json
{
    "providers": [
        {
            "type": "cloudflare",
            "key": "your-api-key",
            "zone": "domain.com",
            "hostname": "hostname.domain.com",
            "ttl": 1,
            "proxied": false
        },
        {
            "type": "namecheap",
            "password": "your-password-key",
            "host": "your-hostname",
            "domain": "your-domain.com"
        },
        {
            "type": "duckdns",
            "token": "your-token",
            "domain": "your-hostname.duckdns.org"
        },
        {
            "type": "porkbun",
            "domain": "your-domain.com",
            "subdomain": "your-subdomain",
            "apikey": "API Key",
            "secretapikey": "Secret Key",
            "ttl": "600"
        }
    ]
}
```

---

## ❖ What's New?
1.1.0 - Add [Porkbun](https://porkbun.com/) support

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
