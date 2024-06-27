# Yoda is [PHP-GTK](https://github.com/scorninpc/php-gtk3) Browser for [Gemini Protocol](https://geminiprotocol.net)

At this moment project under development!

## Protocols

* [x] Gemini
* [x] Nex

## Features

* [x] Custom DNS resolver with memory cache (useful for alt networks like [Yggdrasil](https://github.com/yggdrasil-network/yggdrasil-go))
* [x] Flexible settings in `config.json`, then UI
* [x] Native GTK environment, no custom colors until you change it by `css`
* [x] Multi-tabs
* [x] Navigation history
* [ ] Bookmarks
* [ ] Certificate features
* [ ] Local snaps to make resources accessible even offline
* [ ] `Gemfeed` reader
* [ ] Search engine integrations, probably [Yo!](https://github.com/YGGverse/Yo/tree/gemini) Search by default
* [ ] Machine translations (e.g. [Lingva API](https://github.com/thedaviddelta/lingva-translate))

## Components

* [gemini-php](https://github.com/YGGverse/gemini-php) - PHP 8 library for Gemini protocol
* [gemtext-php](https://github.com/YGGverse/gemtext-php) - PHP 8 library for Gemtext operations
* [net-php](https://github.com/YGGverse/net-php) - PHP 8 library for DNS resolver and address parser