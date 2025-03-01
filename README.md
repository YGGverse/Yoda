# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

GTK 4 / Libadwaita client written in Rust

> [!IMPORTANT]
> Project in development, for stable version use crates.io release!
>

![image](https://github.com/user-attachments/assets/cfbbc3fb-61d2-4afd-a21f-8e36ee329941)

## Features

### Interface
* [x] Multi-tab
* [x] Hotkeys
* [ ] Bookmarks
  * [ ] Browser window
  * [x] Recent bookmarks list
* [ ] Build-in multimedia support
  * [x] [Images](#images)
  * [ ] [Audio](#audio)
  * [ ] [Video](#video)
* [x] Certificates
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
  * [ ] Browser window
  * [x] Page navigation
  * [x] Recently visited
  * [x] Recently closed
* [ ] Proxy
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
* [ ] [NEX](https://nightfall.city/nex/info/specification.txt) - useful for networks with build-in encryption (e.g. [Yggdrasil](https://yggdrasil-network.github.io))
  * [ ] [NPS](https://nightfall.city/nps/info/specification.txt)
* [x] System
  * [x] `file://` - local files browser
* [ ] Request prefix
  * [ ] `about:`
    * [ ] `config` - low-level key/value settings editor
  * [ ] `bookmarks:` - bookmarks browser
  * [ ] `history:` - history browser
  * [x] `download:` - save location to file
  * [x] `source:` - source viewer (by [sourceview5](https://crates.io/crates/sourceview5))

### Media types

#### Text
  * [x] `text/gemini`
  * [x] `text/plain`

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

* Cairo `1.18`
* GdkPixBuf `2.42`
* Glib `2.80`
* Gtk `4.14`
* GtkSourceView `5.14`
* libadwaita `1.5` (Ubuntu 24.04+)
* libspelling `0.1`

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
cargo build
```
* `cargo run`

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

### Releases

* Package version in repository increase after [crates.io](https://crates.io/crates/yoda) release
* Until DB migration not implemented, application will create new profile on every `CARGO_PKG_VERSION_MAJOR`.`CARGO_PKG_VERSION_MINOR` change

### See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) API
* [ggemini](https://github.com/YGGverse/ggemini) - Glib-oriented client for [Gemini protocol](https://geminiprotocol.net/docs/protocol-specification.gmi)