<h1 align="center"> ━━━━  ❖  ━━━━ </h1>

<!-- BADGES -->
<div align="center">
   <p></p>

   <img src="https://img.shields.io/github/stars/dotzenith/zendns?color=F8BD96&labelColor=302D41&style=for-the-badge">

   <img src="https://img.shields.io/github/forks/dotzenith/zendns?color=DDB6F2&labelColor=302D41&style=for-the-badge">

   <img src="https://img.shields.io/github/repo-size/dotzenith/zendns?color=ABE9B3&labelColor=302D41&style=for-the-badge">

   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/zendns?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

## ❖ ZenDNS

ZenDNS is an easy-to-use commandline utility to manage DDNS on [Cloudflare](www.cloudflare.com), [Namecheap](https://www.namecheap.com/), and [DuckDNS](https://www.duckdns.org/)


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
  -c, --config <CONFIG>  The yaml config file to use
  -l, --log <LOGFILE>    Where the output will be logged, uses stdout if not used
  -f, --force            Overrides the check for caching
  -h, --help             Print help
  -V, --version          Print version
```

#### Normal
```sh
zendns --config /path/to/config.yaml
```

#### Dedicated Logfile
```sh
zendns --config /path/to/config.yaml --log /path/to/logfile
```

---

## ❖ Configuration

### ❖ Cloudflare

Create an API token for your zone in [Profile Settings](https://dash.cloudflare.com/profile/api-tokens). The token must have `Zone::DNS::Read` and `Zone::DNS::Edit` permissions.
`Zone Resource` can be set to `Specific Zone` and set to the zone you want to update. If you would like to use this same token for all other zones, please select `All Zones`.
Leave `Client IP Address Filtering` as is, and define how long this token should stay valid for in the `TTL` section.

The configuration for Cloudflare looks as follows:
```yaml
cloudflare:
  - key: "your-api-key"
    zone: "your-website.com"
    hostname: "your-hostname" # `@` if you want to update `your-website.com`
    ttl: 1 # 1 for auto, otherwise between 60 and 86400
    proxied: false
```

### ❖ Namecheap

See [Namecheap's Guide](https://www.namecheap.com/support/knowledgebase/article.aspx/595/11/how-do-i-enable-dynamic-dns-for-a-domain/) on enabling DDNS

The configuration for Namecheap looks as follows:
```yaml
namecheap:
  - password: "your-password-key"
    host: "your-hostname"
    domain: "your-website.com"
```

### ❖ DuckDNS

Copy the `token` from DuckDNS profile page

The configuration for DuckDNS looks as follows:
```yaml
duckdns:
  - token: "your-token"
    domain: "your-hostname.duckdns.org"
```

### ❖ All Together

All of the providers can be added to the same file, with multiple entries per provider as well

```yaml
cloudflare:
  - key: "your-api-key"
    zone: "your-website.com"
    hostname: "your-hostname" # `@` if you want to update `your-website.com`
    ttl: 1 # 1 for auto, otherwise between 60 and 86400
    proxied: false

namecheap:
  - password: "your-password-key"
    host: "your-hostname"
    domain: "your-website.com"

duckdns:
  - token: "your-token"
    domain: "your-hostname.duckdns.org"
  - token: "your-token"
    domain: "your-other-hostname.duckdns.org"
```

---

## ❖ What's New?
0.3.1 - Redo TTL validation and fix logger timestamp

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
