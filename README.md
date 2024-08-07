# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

GTK 4 / C++ implementation branch

> [!IMPORTANT]
> Project in development!
>

## Build

### Linux

* `sudo apt install git libgtk-4-dev`
* `git clone https://github.com/YGGverse/Yoda.git`
* `cd Yoda`
* `git checkout master`
* `make`

#### Update

* `git pull`
* `make clean`
* `make`

## Development

### Environment

* `pkg-config --cflags --libs gtk4`

### Contribution

* `cd Yoda`
* `git pull`
* `git checkout master`
* `git checkout -b 'contribution-name'`