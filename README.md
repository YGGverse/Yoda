# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

C++ / GTK 4 implementation, see also [PHP-GTK3](https://github.com/YGGverse/Yoda/tree/PHP-GTK3) experimental branch

> [!IMPORTANT]
> Project in development!
>

## Build

### Linux

``` bash
apt install git\
            libglib2.0-dev\
            libglibmm-2.68-dev\
            libgtkmm-4.0-dev\
            libpangomm-2.48-dev\
            libsqlite3-dev
```

* `git clone https://github.com/YGGverse/Yoda.git`
* `cd Yoda`
* `git checkout CPP-GTK4`
* `make`

#### Update

* `git pull`
* `make clean`
* `make`

## Localization

### Official

 * [Crowdin](https://crowdin.com/project/yoda-browser)

### Personal

* `apt install intltool`
* `cd po`
* `intltool-update --pot`

## Development

### Environment

``` bash
pkg-config --cflags --libs gio-2.0\
                           glibmm-2.68\
                           gtkmm-4.0\
                           pangomm-2.48\
                           sqlite3
```

### Contribution

* `cd Yoda`
* `git checkout CPP-GTK4`
* `git pull`
* `git checkout -b 'contribution-name'`

### Documentation

#### Components

* [GTK](https://gtk.org) - free and open-source cross-platform widget toolkit
  * [gtkmm](https://gtkmm.org) - official C++ interface for GTK
* [SQLite](https://sqlite.org) - profile database