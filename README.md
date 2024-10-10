# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

Rust / GTK 4 implementation

> [!IMPORTANT]
> Project in development!
>

## Build

### Dependencies

Make sure system support:

* GTK 4.8+
* Libadwaita 1.4+

Use [rustup installer](https://rustup.rs) to setup latest Rust compiler and package manager:

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
git checkout Rust-GTK4
cargo build
```

## Development

Guide and protocol draft

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
  * encapsulate it members: compose childs and stay composable for parents
  * access 1 level of childs, never parents (e.g. through `super`)
  * implement only one public API `struct` per file (same as one file for one class)
    * implementable `struct` is public, where it members - private
  * contain main `struct` implementation:
    * at least one constructor that must:
      * have common for application name: `new` or/and `new_arc`, `new_mutex`, etc - on return object in container
      * grant ownership for new `Self` object created
    * public `activate` action if the new object can not be activated on construct
    * public `link` getter for GTK `widget` (parental composition)
* Public API oriented to simple (`integer`, `boolean`), standard (`std::*`) or system-wide (`gio`, `glib`, etc) data types usage to reduce internal dependencies from app implementation

#### Database

* [SQLite](https://sqlite.org) used to operate with user profile: for example, restore and save widget sessions, manage auth, history, bookmarks, etc
* Database stored in system config directory (could be detected simply using browser tools menu)
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

### Releases

* Package version in repository should be increased immediately after stable release on [crates.io](https://crates.io/crates/yoda) and before apply new changes
* Currently, profile data stored in separated sub-directory, for every new release version

## See also

* [CPP-GTK4](https://github.com/YGGverse/Yoda/tree/CPP-GTK4) - C++ / GTK 4 implementation
* [PHP-GTK3](https://github.com/YGGverse/Yoda/tree/PHP-GTK3) - PHP / GTK 3 experimental branch