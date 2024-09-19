# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

> [!IMPORTANT]
> This is experimental PHP / GTK3 branch,
> checkout [Rust-GTK4](https://github.com/YGGverse/Yoda/tree/Rust-GTK4) or [CPP-GTK4](https://github.com/YGGverse/Yoda/tree/CPP-GTK4) for alternative implementations
>

## Install

### Auto

* `git clone https://github.com/YGGverse/Yoda.git`
* `cd Yoda`
* `git checkout PHP-GTK3`
* Run `./INSTALL.sh` to auto-build Yoda for Linux systems, using latest [PHP](https://github.com/php/php-src), [PHP-CPP](https://github.com/fast-debug/PHP-CPP) and [PHP-GTK3](https://github.com/scorninpc/php-gtk3)

Tool does not change global PHP version, not requires root permissions, but optionally install system dependencies, through `apt` or `yum`

#### After build

* To **launch**, open Yoda in application menu or use `yoda.sh` launcher generated
* To **update**, run `INSTALL.sh` again
* To **uninstall**, just remove `build` folder with profile data in `~/.yoda`

### Manual

#### Environment

``` bash
apt install autoconf\
            automake\
            bison\
            build-essential\
            composer\
            git\
            libbz2-dev\
            libcurl4-openssl-dev\
            libfreetype6-dev\
            libgladeui-dev\
            libgtk-3-dev\
            libgtksourceview-3.0-dev\
            libjpeg-dev\
            libonig-dev\
            libpng-dev\
            libpq-dev\
            libreadline-dev\
            libsqlite3-dev\
            libssl-dev\
            libtool\
            libwebp-dev\
            libwnck-dev\
            libxml2-dev\
            libxpm-dev\
            libxslt1-dev\
            libzip-dev\
            re2c
```

#### [PHP](https://github.com/php/php-src)

Make sure version 8.1 or above installed and configured with following options:

```
./configure --enable-mbstring\
            --enable-pcntl\
            --enable-shmop\
            --enable-sockets\
            --enable-static\
            --with-gettext\
            --with-openssl\
            --with-pdo-sqlite
```

#### [PHP-CPP](https://github.com/fast-debug/PHP-CPP)

Use official [documentation](https://www.php-cpp.com/documentation) for details.

Build from source:

* `git clone https://github.com/fast-debug/php-cpp.git`
* `cd php-cpp`
* `make && sudo make install`

#### [PHP-GTK](https://github.com/scorninpc/php-gtk3)

Follow [installation guide](https://github.com/scorninpc/php-gtk3#acknowledgements) or use [binaries](https://github.com/scorninpc/php-gtk3/releases).

Build from source:

* `git clone https://github.com/fast-debug/php-gtk3.git`
* `cd php-gtk3`
* `make && sudo make install`

#### Yoda

* `git clone https://github.com/YGGverse/Yoda.git`
* `cd Yoda`
* `composer install`

#### Launch

``` bash
php -dextension=php-gtk3.so src/Yoda.php
```

## Components

* [gemini-php](https://github.com/YGGverse/gemini-php) - Gemini protocol connections
* [gemtext-php](https://github.com/YGGverse/gemtext-php) - Gemtext operations
* [net-php](https://github.com/YGGverse/net-php) - DNS resolver and network address features
* [nex-php](https://github.com/YGGverse/nex-php) - NEX protocol connections