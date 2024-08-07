# Yoda - Browser for [Gemini protocol](https://geminiprotocol.net)

GTK 4 / C++ implementation branch

> [!IMPORTANT]
> Project in development!
>

## Install

### Source

#### Linux

##### Dependencies

``` bash
sudo apt install git libgtk-4-dev
```

##### Build

* `git clone https://github.com/YGGverse/Yoda.git`
* `cd Yoda`
* `git checkout master`
* `make`

###### Update

* `cd Yoda`
* `git checkout master`
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