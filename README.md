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

* Main file must have `mod.rs` name because it may include additional submodules in same location like database, template, etc
* The codebase of module must be as minimal as possible: separate different tasks to submodules
* Every module or it helper must contain and implement only one `struct`
* Composition modules (like box for window) stored in sub-folders and following same principles as parent
* Every mod access 1 level of child API, never parents (e.g.`super`)
* Mod constructor must return [Arc pointer](https://doc.rust-lang.org/std/sync/struct.Arc.html) for new object created (to support async operations)

### Contribution

* before commit, make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`

## See also

* [CPP-GTK4](https://github.com/YGGverse/Yoda/tree/CPP-GTK4) - C++ / GTK 4 implementation
* [PHP-GTK3](https://github.com/YGGverse/Yoda/tree/PHP-GTK3) - PHP / GTK 3 experimental branch