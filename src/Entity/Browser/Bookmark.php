<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \GtkWindow;

use \Yggverse\Yoda\Entity\Browser;

class Bookmark
{
    // GTK
    public \GtkWindow $gtk;

    // Dependencies
    public Browser $browser;

    // Requirements
    public Bookmark\Header $header;
    public Bookmark\Container $container;

    // Defaults
    public const WIDTH = 640;
    public const HEIGHT = 640;
    public const MAXIMIZE = false;

    public function __construct(
        Browser $browser
    ) {
        // Init dependencies
        $this->browser = $browser;

        // Init window
        $this->gtk = new GtkWindow;

        $this->gtk->set_size_request(
            $this::WIDTH,
            $this::HEIGHT
        );

        if ($this::MAXIMIZE)
        {
            $this->gtk->maximize();
        }

        // Init header
        $this->header = new Bookmark\Header(
            $this
        );

        $this->gtk->set_titlebar(
            $this->header->gtk
        );

        // Init container
        $this->container = new Bookmark\Container(
            $this
        );

        $this->gtk->add(
            $this->container->gtk
        );

        // Render
        $this->gtk->show();
    }
}