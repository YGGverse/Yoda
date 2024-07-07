<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \Yggverse\Yoda\Entity\Browser\History\Header;
use \Yggverse\Yoda\Entity\Browser\History\Container;

class History
{
    public \GtkWindow $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser $browser;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\History\Header $header;
    public \Yggverse\Yoda\Entity\Browser\History\Container $container;

    // Defaults
    private int $_width  = 640;
    private int $_height = 480;
    private bool $_maximize = false;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser $browser
    ) {
        // Init dependencies
        $this->browser = $browser;

        // Init window
        $this->gtk = new \GtkWindow;

        $this->gtk->set_size_request(
            $this->_width,
            $this->_height
        );

        if ($this->_maximize)
        {
            $this->gtk->maximize();
        }

        // Init header
        $this->header = new Header(
            $this
        );

        $this->gtk->set_titlebar(
            $this->header->gtk
        );

        // Init container
        $this->container = new Container(
            $this
        );

        $this->gtk->add(
            $this->container->gtk
        );

        // Render
        $this->gtk->show();
    }
}