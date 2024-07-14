# Yoda is [PHP-GTK](https://github.com/scorninpc/php-gtk3) Browser for [Gemini protocol](https://geminiprotocol.net)

Project in development!

## Install

### Builder

Run `INSTALL.sh` from terminal to build the app for Linux systems using latest [PHP](https://github.com/php/php-src), [PHP-CPP](https://github.com/fast-debug/PHP-CPP) and [PHP-GTK3](https://github.com/scorninpc/php-gtk3)

This tool does not change global PHP version, not requires root permissions, but optionally install system dependencies, through `apt`

* To start, open Yoda in application menu or use `yoda.sh` launcher created
* To uninstall, just remove `build` folder generated in Yoda directory, remove profile data `~/.yoda`

### Manual

1. Build latest [PHP-GTK3](https://github.com/scorninpc/php-gtk3) or get [binaries](https://github.com/scorninpc/php-gtk3/releases)
2. `apt install git composer`
3. `git clone https://github.com/YGGverse/Yoda.git`
4. `cd Yoda`
5. `composer update`

#### Launch

``` bash
/path/to/php-gtk3 src/Yoda.php
```

## Components

* [gemini-php](https://github.com/YGGverse/gemini-php) - Gemini protocol connections
* [gemtext-php](https://github.com/YGGverse/gemtext-php) - Gemtext operations
* [net-php](https://github.com/YGGverse/net-php) - DNS resolver and network address features
* [nex-php](https://github.com/YGGverse/nex-php) - NEX protocol connections