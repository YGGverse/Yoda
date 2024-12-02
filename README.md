# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

GTK 4 / Libadwaita client written in Rust

> [!IMPORTANT]
> Project in development!
>

![image](https://github.com/user-attachments/assets/cfbbc3fb-61d2-4afd-a21f-8e36ee329941)

## Features

### Interface
* [x] Multi-tab
* [x] Hotkeys
* [ ] Bookmarks
* [ ] Build-in multimedia support
  * [x] [Images](#images)
  * [ ] [Audio](#audio)
  * [ ] [Video](#video)
* [ ] Certificates
* [ ] Downloads
* [ ] History
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
  * [ ] [Status code](https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes)
    * [x] Success
      * [x] `20`
    * [x] Input
      * [x] `10` Input
      * [x] `11` Sensitive input
    * [x] Redirection
      * [x] `30` Temporary
      * [x] `31` Permanent
    * [ ] Temporary failure
      * [ ] `40` Unspecified condition
      * [ ] `41` Server unavailable
      * [ ] `42` CGI error
      * [ ] `43` Proxy error
      * [ ] `44` Slow down
    * [ ] Permanent failure
      * [ ] `50` General
      * [ ] `51` Not found
      * [ ] `52` Gone
      * [ ] `53` Proxy request refused
      * [ ] `59` Bad request
    * [x] Client certificates
      * [x] `60` Certificate requested
      * [x] `61` Certificate not authorized
      * [x] `62` Certificate not valid
  * [x] [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) (by [ggemtext](https://github.com/YGGverse/ggemtext))
    * [x] Code (inline/multiline)
      * [x] Inline
      * [x] Multiline
        * [x] Alt
      * [ ] Terminal emulation*
      * [ ] Syntax highlight*
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
  * [ ] Titan
* [ ] [NEX](https://nightfall.city/nex/info/specification.txt) - useful for networks with build-in encryption (e.g. [Yggdrasil](https://yggdrasil-network.github.io))
  * [ ] [NPS](https://nightfall.city/nps/info/specification.txt)
* Localhost
  * [ ] `file://` - localhost browser
  * [ ] `config://` - low-level key/value settings editor
  * [ ] `view-source://` - page source viewer (where supported)

### Media types

#### Text
  * [x] `text/gemini`
  * [ ] `text/plain`

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

### Requirements

* Cairo `1.16`
* GDK PixBuf `2.42`
* Gio `2.82`
* Glib `2.56`
* GTK `4.16`
* Libadwaita `1.6`

Use [rustup](https://rustup.rs) installer to setup latest Rust compiler and Cargo package manager:

``` bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Debian

_todo_

#### Fedora

``` bash
sudo dnf install git gcc\
                 cairo-devel glib2-devel gtk4-devel libadwaita-devel pango-devel\
                 sqlite-devel
```

### Install

#### Stable

``` bash
cargo install Yoda
```

#### Repository

``` bash
git clone https://github.com/YGGverse/Yoda.git
cd Yoda
cargo build
```

## Development

### Contribution

* Before commit, please make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`
  * run `cargo clippy` for final optimization

#### Contributors

![wakatime](https://wakatime.com/badge/user/0b7fe6c1-b091-4c98-b930-75cfee17c7a5/project/018ebca8-4d22-4f9e-b557-186be6553d9a.svg) ![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)

### Releases

* Package version in repository increase after [crates.io](https://crates.io/crates/yoda) release
* Until DB migration not implemented, application will create new profile on `CARGO_PKG_VERSION_MAJOR`.`CARGO_PKG_VERSION_MINOR` change

### See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) API
* [ggemini](https://github.com/YGGverse/ggemini) - Glib-oriented client for [Gemini protocol](https://geminiprotocol.net/docs/protocol-specification.gmi)