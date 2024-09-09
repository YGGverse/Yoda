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

### Introduction

Project structure and codebase in development, it promise to grow. This section would help with understand what is going on, before you start to work with code.

Currently, application sources located at `src/app` folder, written by following principles:

* Every file implement (extend) one of GTK 4 Widgets, according to the functional subject (e.g. `class Browser : public Gtk::ApplicationWindow`)
* `src/app` does not contain other class types like models or libraries (another folders and namespaces at `src` root are reserved for these needs)
* Namespaces match filesystem path, where directory namespaces are lowercase
* Every file work with it own, 1th level child only, to prevent massive logic levels keeping in mind
* To access any children features, deeper or higher than 1th level of current class, use delegation methods (actions, getters and setters)
* One file - one class. If the file requires additional (GTK) component, this component should be placed at similar folder with same name as parent filename. So we have simple hierarchy navigation logic - from app to window, from window to it container, etc.
* At this moment, all constants named with uppercase, const everything that not mutable
* `#include` application `.hpp` files in `.cpp`. For system libraries, use headers only. Do not place system dependencies in `.cpp`

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