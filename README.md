# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

Rust / GTK 4 implementation

> [!IMPORTANT]
> Project in development!
>

## Install

### Stable

``` bash
cargo install Yoda
```

### Repository

``` bash
git clone https://github.com/YGGverse/Yoda.git
cd Yoda
git checkout Rust-GTK4
cargo run
```

## Development

Guide and protocol draft

### `browser`

#### Filesystem

* Use [modern path pattern](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#no-more-modrs)
* One module implements one GTK widget, it may include additional helper files in same location (like template, CSS or DB API)
* For children widget - create children module, located according to hierarchy

#### Codebase

* The codebase must be as minimal as possible, separate:
  * different tasks
  * massive structures
  * structures with implementation
* Every module must:
  * encapsulate it members: compose childs and stay composable for parents
  * access 1 level of childs, never parents (e.g. through `super`)
  * implement only one `struct` (same as one file for one class)
    * implementable `struct` is public, where members - private
  * contain main `struct` implementation:
    * at least one constructor that must return:
      * unwrapped main `Self` structure
      * granted ownership for new object created
    * public link getter for privately constructed widget
* Public API oriented to simple (`integer`, `boolean`), standard (`std::*`) or system-wide (`gio`, `glib`, etc) data types usage to reduce internal dependencies from app implementation

#### GTK

* Operate with [action objects](https://docs.gtk.org/gio/class.SimpleAction.html) instead of names like `win.action`. This allows to follow encapsulation, because by the goal, module must know nothing about parent presets - for example, define some action in parent, then delegate object created as argument

### Contribution

* before commit, make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`

## See also

* [CPP-GTK4](https://github.com/YGGverse/Yoda/tree/CPP-GTK4) - C++ / GTK 4 implementation
* [PHP-GTK3](https://github.com/YGGverse/Yoda/tree/PHP-GTK3) - PHP / GTK 3 experimental branch