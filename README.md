# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

GTK 4 / C++ implementation branch

> [!IMPORTANT]
> Project in development!
>

## Build

### Linux

``` bash
apt install git \
            libgtkmm-4.0-dev \
            libglibmm-2.68-dev \
            libsqlite3-dev
```

* `git clone https://github.com/YGGverse/Yoda.git`
* `cd Yoda`
* `git checkout master`
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
pkg-config --cflags --libs gtkmm-4.0 \
                           glibmm-2.68 \
                           sqlite3
```

### Contribution

* `cd Yoda`
* `git checkout master`
* `git pull`
* `git checkout -b 'contribution-name'`

### Documentation

#### Components

* [GTK](https://gtk.org) - free and open-source cross-platform widget toolkit
  * [gtkmm](https://gtkmm.org) - official C++ interface for GTK
* [SQLite](https://sqlite.org) - profile database