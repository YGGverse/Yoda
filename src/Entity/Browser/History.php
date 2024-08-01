<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \GtkWindow;
use \GtkWindowType;

use \Yggverse\Yoda\Entity\Browser;

class History
{
    // GTK
    public \GtkWindow $gtk;

    // Dependencies
    public Browser $browser;

    // Requirements
    public History\Header $header;
    public History\Container $container;

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
        $this->gtk = new GtkWindow(
            GtkWindowType::TOPLEVEL // GtkWindowType::POPUP
        );

        $this->gtk->set_size_request(
            $this::WIDTH,
            $this::HEIGHT
        );

        if ($this::MAXIMIZE)
        {
            $this->gtk->maximize();
        }

        // Init header
        $this->header = new History\Header(
            $this
        );

        $this->gtk->set_titlebar(
            $this->header->gtk
        );

        // Init container
        $this->container = new History\Container(
            $this
        );

        $this->gtk->add(
            $this->container->gtk
        );

        // Render
        $this->gtk->show();
    }
}