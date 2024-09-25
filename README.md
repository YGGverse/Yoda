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

* Composition pattern, using standard `mod.rs` model
* One mod = one widget. Create new mod for new widget implementation
* The codebase of module must be as minimal as possible: separate different tasks to sub-modules
* Every module must:
  * access 1 level of child API, never parents (e.g.`super`)
  * implement only one `struct` (same as one file for one class)
    * `struct` is public, where members - private
  * contain `struct` implementation:
    * at least one constructor that must return:
      * raw `struct` (`new() -> Self`) without cover to any kind of smart `std` or `glib` pointers
      * ownership to new object created
    * one link getter for widget implementation (`widget -> &Gtk`) because it's private

### Contribution

* before commit, make sure:
  * new branch created for every new PR `git checkout -b 'contribution-name'`
  * new code follows common [rustfmt](https://rust-lang.github.io/rustfmt/) style `cargo fmt --check`

## See also

* [CPP-GTK4](https://github.com/YGGverse/Yoda/tree/CPP-GTK4) - C++ / GTK 4 implementation
* [PHP-GTK3](https://github.com/YGGverse/Yoda/tree/PHP-GTK3) - PHP / GTK 3 experimental branch