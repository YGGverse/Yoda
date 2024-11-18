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
    * [ ] Client certificates
      * [ ] `60` Certificate requested
      * [ ] `61` Certificate not authorized
      * [ ] `62` Certificate not valid
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
* [ ] `file://` - localhost browser
* [ ] `config://` - low-level key/value settings editor

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
* Gio `2.82`
* Glib `2.56`
* GTK `4.16`
* Libadwaita `1.6`
* PixBuf `2.42`

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

Quick start guide and maintenance protocol

### `browser`

#### Filesystem

* Use [modern path pattern](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#no-more-modrs)
* One module implements one GTK widget, it may include additional helper files in same location (like template, CSS or DB API)
* For children widget - create children module, located according to hierarchy

#### Codebase

* Every module should be as minimal as possible, separate:
  * different tasks
  * massive structures
  * structures with implementation
* Every module must:
  * encapsulate members - use objects, not static names (unlike native GTK actions API)
  * implement only one public API `struct` per file (same as one file for one class)
    * implementable `struct` is public, where it members - private
  * contain main `struct` implementation:
    * at least one constructor that must:
      * have common for application names: `from`, `new` or/and `new_rc`, `new_mutex`, etc - on return object in container
      * grant ownership for new `Self` object created
    * public `activate` action if the new object can not be activated on construct
    * public `link` getter for GTK `widget` (parental composition)
* Public API oriented to simple (`integer`, `boolean`), standard (`std::*`) or system-wide (`gio`, `glib`, etc) data types usage to reduce internal dependencies from app implementation

#### Database

* [SQLite](https://sqlite.org) used to operate with user profile: for example, restore and save widget sessions, manage auth, history, bookmarks, etc
* Database stored in system config directory (could be detected simply using browser tools menu)
* Structure of table should not be modified on `CARGO_PKG_VERSION_PATCH` change
* Every `browser` mod may have own table, where table must:
  * contain same name as mod location, for example `app_browser_widget` for `src/app/browser/widget.rs`
  * every table include autoincrement `id` column and parental primary ID if exist
    * column name for parental ID must have absolute namespace prefix, for example `app_browser_id` column for `app_browser_widget` table. For example, if the table has few parental keys, column set could be `id`, `parent_one_id`, `parent_two_id`, `some_data`
* _todo_
  * [ ] version control for auto-migrations
  * [x] transactions support for update operations

#### GTK

* Operate with [action objects](https://docs.gtk.org/gio/class.SimpleAction.html) instead of names like `win.action`. This allows to follow encapsulation, by the our goal, module must know nothing about parent presets. For example, define some action in parent, then delegate object created as construction argument
* Started refactory on separate widgets implementation to separated mods, because widgets may contain own tables in database and require additional mods dependencies like ORM API _todo_

### Contribution

* Before commit, please make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`
  * run `cargo clippy` for final optimization

#### Contributors

![wakatime](https://wakatime.com/badge/user/0b7fe6c1-b091-4c98-b930-75cfee17c7a5/project/018ebca8-4d22-4f9e-b557-186be6553d9a.svg) ![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)

### Releases

* Package version in repository should be increased immediately after stable release on [crates.io](https://crates.io/crates/yoda) and before apply new changes
* Currently, profile data stored in separated sub-directories, auto-created on every `CARGO_PKG_VERSION_MAJOR` or/and `CARGO_PKG_VERSION_MINOR` change

### See also

* [ggemtext](https://github.com/YGGverse/ggemtext) - Glib-oriented [Gemtext](https://geminiprotocol.net/docs/gemtext-specification.gmi) API
* [ggemini](https://github.com/YGGverse/ggemini) - Glib-oriented client for [Gemini protocol](https://geminiprotocol.net/docs/protocol-specification.gmi)