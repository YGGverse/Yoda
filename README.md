# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

GTK 4 / C++ implementation branch

> [!IMPORTANT]
> Project in development!
>

## Build

### Linux

* `sudo apt install git libgtkmm-4.0-dev libglibmm-2.68-dev`
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

* `sudo apt install intltool`
* `cd po`
* `intltool-update --pot`

## Development

### Environment

* `pkg-config --cflags --libs gtkmm-4.0 glibmm-2.68`

### Contribution

* `cd Yoda`
* `git checkout master`
* `git pull`
* `git checkout -b 'contribution-name'`

### Documentation

#### Components

* [GTK](https://gtk.org)
* [gtkmm](https://gtkmm.org)