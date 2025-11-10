# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

<img src="https://raw.githubusercontent.com/YGGverse/Yoda/refs/heads/master/data/io.github.yggverse.Yoda.svg" alt="Yoda browser logo" width="160">

Privacy-oriented GTK 4 / Libadwaita client written in Rust.

The term _Privacy-oriented_ means that Yoda complies to the [Gemini protocol specification](https://geminiprotocol.net/docs/protocol-specification.gmi) and excludes third-party connections, that making it safe to use in combination with I2P. It also includes useful tools, such as connection details, optional DNS/Geo-IP features, flexible proxy configuration for use with modern IPv6 mesh networks like Yggdrasil, Mycelium, CJDNS, and others.

Yoda browser is primarily designed by and for experienced network users who care about their fingerprints and prefer to control every action manually. It does not preload tab content on app opening, does not run any background connections, does not incorporate web-like media preloading without user initiation, and does not automatically check for updates, even from 'official' servers. Additionally, it prevents auto-follow external redirection by default and requires manual confirmation, which is currently not clearly specified.

The Gemini protocol was designed as a minimalistic, tracking-resistant alternative to the Web, and Yoda embraces this philosophy by providing a straightforward graphical user interface (GUI) that is partially inspired by the Firefox UI, making it intuitively comfortable for regular users.

> [!IMPORTANT]
> Project in development, for stable version use checkpoint [releases](https://github.com/YGGverse/Yoda/releases)!
>

![image](https://github.com/user-attachments/assets/cfbbc3fb-61d2-4afd-a21f-8e36ee329941)

## Features

### Interface
* [x] Multi-tab
* [x] Hotkeys
* [x] Bookmarks
* [ ] Build-in multimedia support
  * [x] [Images](#images)
  * [ ] [Audio](#audio)
  * [ ] [Video](#video)
* [x] Certificates
  * [x] Server
    * [x] The [TOFU](https://en.wikipedia.org/wiki/Trust_on_first_use) validation
  * [x] Client
    * [x] Generate new identity
    * [x] Select for path
    * [x] Export to PEM
    * [x] Import from PEM
    * [x] Delete
* [x] Custom search providers
* [ ] Downloads
  * [ ] Browser window
  * [x] Save page as file
  * [x] Unsupported content type downloads
* [ ] History
  * [x] Recently visited
  * [ ] Recently closed
* [x] Proxy (by [SimpleProxyResolver](https://docs.gtk.org/gio/class.SimpleProxyResolver.html))
  * [x] Multiple regex rules by the priority
  * [x] Custom ignored hosts
  * [x] UI indication with the accent colors
* [ ] Session
  * [ ] Window
    * [x] Size
    * [x] Tabs
      * [x] Pin
      * [x] Page
        * [ ] Content (cache)
        * [x] Meta
          * [x] Title
        * [ ] Navigation
          * [x] Request
          * [ ] History
* [ ] User settings

### Protocols
* [ ] [Gemini](https://geminiprotocol.net/docs/protocol-specification.gmi)
  * [x] [Status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    * [x] Success
      * [x] `20`
    * [x] Input
      * [x] `10` Input
      * [x] `11` Sensitive input
    * [x] Redirection
      * [x] `30` Temporary
      * [x] `31` Permanent
    * [x] Temporary failure
      * [x] `40` Unspecified condition
      * [x] `41` Server unavailable
      * [x] `42` CGI error
      * [x] `43` Proxy error
      * [x] `44` Slow down
    * [x] Permanent failure
      * [x] `50` General
      * [x] `51` Not found
      * [x] `52` Gone
      * [x] `53` Proxy request refused
      * [x] `59` Bad request
    * [x] Client certificates
      * [x] `60` Certificate requested
      * [x] `61` Certificate not authorized
      * [x] `62` Certificate not valid
  * [x] [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) (by [ggemtext](https://crates.io/crates/ggemtext))
    * [x] Code (inline/multiline)
      * [x] Inline
      * [x] Multiline
        * [x] Alt
      * [x] Syntax highlight* (by [syntect](https://crates.io/crates/syntect))
      * [ ] Terminal emulation* (by [ansi-parser](https://crates.io/crates/ansi-parser))
          * [x] foreground
          * [x] background
          * [ ] intensity
          * [ ] italic
          * [ ] underline
          * [ ] blink
          * [ ] reversed
          * [ ] strikethrough
    * [x] Header
      * [x] H1
      * [x] H2
      * [x] H3
    * [x] Link
      * [x] Address
      * [x] Date
      * [x] Alt
    * [x] List
    * [x] Quote
  * [ ] Gemfeed
    * [ ] by headers
    * [ ] by links
    * [ ] by content hash*
  * [x] [Titan](https://transjovian.org/titan/page/The%20Titan%20Specification)
    * [x] Binary data (file uploads)
    * [x] Text input
    * [x] Header options
      * [x] MIME
      * [x] Token
* [x] [NEX](https://nightfall.city/nex/info/specification.txt) - useful for networks with build-in encryption (e.g. [Yggdrasil](https://yggdrasil-network.github.io) or [Mycelium](https://github.com/threefoldtech/mycelium))
  * [ ] [NPS](https://nightfall.city/nps/info/specification.txt)
* [x] System
  * [x] `file://` - local files browser
* [x] Request prefix
  * [x] `download:` - save location to file
  * [x] `source:` - source viewer (by [sourceview5](https://crates.io/crates/sourceview5))

### Media types

#### Text
  * [x] `text/gemini`
  * [x] `text/plain`
  * [x] `text/nex`

#### Images
  * [x] `image/gif`
  * [x] `image/jpeg`
  * [x] `image/png`
  * [x] `image/webp`
  * [ ] `image/svg+xml`

#### Audio
  * [ ] `audio/flac`
  * [ ] `audio/mpeg`
  * [ ] `audio/ogg`

#### Video

## Build

![Linux](https://github.com/YGGverse/Yoda/actions/workflows/linux.yml/badge.svg)
[![Dependencies](https://deps.rs/repo/github/YGGverse/Yoda/status.svg)](https://deps.rs/repo/github/YGGverse/Yoda)
[![crates.io](https://img.shields.io/crates/v/Yoda.svg)](https://crates.io/crates/Yoda)

### Requirements

* Cairo `1.18+`
* GdkPixBuf `2.42+`
* Glib `2.80+`
* Gtk `4.14+`
* GtkSourceView `5.14+`
* libadwaita `1.5+` (Ubuntu 24.04+)
* libspelling `0.1+`

#### Debian

``` bash
sudo apt install git curl build-essential\
                 libgtk-4-dev libgtksourceview-5-dev libglib2.0-dev libadwaita-1-dev libspelling-1-dev\
                 libsqlite3-dev libssl-dev
```

#### Fedora

``` bash
sudo dnf install git curl gcc\
                 gtk4-devel gtksourceview5-devel glib2-devel libadwaita-devel libspelling-devel\
                 sqlite-devel openssl-devel
```

#### Rust

Use [rustup](https://rustup.rs) installer to setup latest Rust compiler and Cargo package manager:

``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install

#### Stable

``` bash
cargo install Yoda
```
* run `Yoda`

#### Repository

``` bash
git clone https://github.com/YGGverse/Yoda.git
cd Yoda
cargo build --release
```
* run `target/release/Yoda`

#### Flatpak

``` bash
git clone https://github.com/YGGverse/Yoda.git
cd Yoda
flatpak-builder --force-clean build\
                --install-deps-from=flathub\
                --install --repo=repo --user\
                io.github.yggverse.Yoda.yaml
```
* launch: `flatpak run io.github.yggverse.Yoda`
* bundle: `flatpak build-bundle repo Yoda.flatpak io.github.yggverse.Yoda`

## Development

### Contribution

* Before commit, please make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`
  * run `cargo clippy` for final optimization

#### Contributors

![wakatime](https://wakatime.com/badge/user/0b7fe6c1-b091-4c98-b930-75cfee17c7a5/project/018ebca8-4d22-4f9e-b557-186be6553d9a.svg) ![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)

### Localization

UI localization planed as Crowdin repository, but it is not implemented yet.

To install new spell dictionaries in Fedora, use `sudo dnf install hunspell-CODE`
* just replace `CODE` with the locale code you want

### Geo-location

To enable geo-location features, get official IP database from [MaxMind](https://www.maxmind.com)\
then copy following files into the `config` folder (available from menu)

* `GeoLite2-Country.mmdb`
* `GeoLite2-City.mmdb` (not implemented yet)

### Releases

* Package version in repository increase after [crates.io](https://crates.io/crates/yoda) release
* Until DB migration not implemented, application will create new profile on every `CARGO_PKG_VERSION_MAJOR`.`CARGO_PKG_VERSION_MINOR` change

### See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) API
* [ggemini](https://github.com/YGGverse/ggemini) - Glib-oriented client for [Gemini protocol](https://geminiprotocol.net/docs/protocol-specification.gmi)