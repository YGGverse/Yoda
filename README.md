# Yoda is [PHP-GTK](https://github.com/scorninpc/php-gtk3) Browser for [Gemini protocol](https://geminiprotocol.net)

Project in development!

## Install

### Builder

Run `INSTALL.sh` in terminal to autobuild Yoda for Linux systems, using latest [PHP](https://github.com/php/php-src), [PHP-CPP](https://github.com/fast-debug/PHP-CPP) and [PHP-GTK3](https://github.com/scorninpc/php-gtk3)

This tool does not change global PHP version, not requires root permissions, but optionally install system dependencies, through `apt` or `yum`

#### After build

* To **launch**, open Yoda in application menu or use `yoda.sh` launcher generated
* To **update**, run `INSTALL.sh` again
* To **uninstall**, just remove `build` folder with profile data in `~/.yoda`

### Manual

#### Environment

``` bash
apt install libpq-dev\
            bison\
            libreadline-dev\
            git\
            composer\
            build-essential\
            autoconf\
            automake\
            libtool\
            re2c\
            libxml2-dev\
            libcurl4-openssl-dev\
            libssl-dev\
            libbz2-dev\
            libjpeg-dev\
            libpng-dev\
            libxpm-dev\
            libfreetype6-dev\
            libzip-dev\
            libsqlite3-dev\
            libonig-dev\
            libxslt1-dev\
            libgtk-3-dev\
            libgladeui-dev\
            libgtksourceview-3.0-dev\
            libwnck-dev
```

#### PHP

Make sure [PHP](https://github.com/php/php-src) version is 8.1 or above and configured with following components:

```
./configure --with-openssl\
            --with-gettext\
            --with-pdo-mysql\
            --enable-sockets\
            --enable-mbstring\
            --enable-shmop\
            --enable-pcntl
```

#### PHP-CPP

PHP-GTK require [PHP-CPP](https://github.com/fast-debug/PHP-CPP) extension to interact native GTK3 libraries.

Use official [documentation](https://www.php-cpp.com/documentation) for details.

#### PHP-GTK

Build latest [PHP-GTK3](https://github.com/scorninpc/php-gtk3) or get binaries.

Follow official [guide](https://github.com/scorninpc/php-gtk3#acknowledgements) for details.

#### Yoda

1. `git clone https://github.com/YGGverse/Yoda.git`
2. `cd Yoda`
3. `composer update`

#### Launch

``` bash
/path/to/php-gtk3 src/Yoda.php
```

## Components

* [gemini-php](https://github.com/YGGverse/gemini-php) - Gemini protocol connections
* [gemtext-php](https://github.com/YGGverse/gemtext-php) - Gemtext operations
* [net-php](https://github.com/YGGverse/net-php) - DNS resolver and network address features
* [nex-php](https://github.com/YGGverse/nex-php) - NEX protocol connections