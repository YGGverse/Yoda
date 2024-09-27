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

This guide in process

### Modules

#### `browser`

* Use [modern path pattern](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#no-more-modrs)
* One module implements one GTK widget, it may include additional helper files in same location (like template, CSS or DB API)
* For children widget - create children module, located according to hierarchy
* The codebase of module must be as minimal as possible, separate to sub-modules:
  * different tasks
  * massive structures
  * structures with implementation
* Every module must:
  * encapsulate: compose childs and stay composable for parents
  * access 1 level of child API, never parents (e.g. through `super`)
  * implement only one `struct` (same as one file for one class)
    * `struct` is public, where members - private
  * contain main `struct` implementation:
    * at least one constructor that must return:
      * unwrapped main `Self` structure
      * granted ownership for new object created
    * public link getter for privately constructed widget

### Contribution

* before commit, make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`

## See also

* [CPP-GTK4](https://github.com/YGGverse/Yoda/tree/CPP-GTK4) - C++ / GTK 4 implementation
* [PHP-GTK3](https://github.com/YGGverse/Yoda/tree/PHP-GTK3) - PHP / GTK 3 experimental branch