<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity;

use \Yggverse\Yoda\Entity\Browser\Header;
use \Yggverse\Yoda\Entity\Browser\Container;

class Browser
{
    public \GtkWindow $gtk;

    // Dependencies
    public \Yggverse\Yoda\Model\Database $database;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Header $header;
    public \Yggverse\Yoda\Entity\Browser\Container $container;

    // Defaults
    private int $_width  = 640;
    private int $_height = 480;
    private bool $_maximize = true;

    public function __construct(
        \Yggverse\Yoda\Model\Database $database
    ) {
        // Init dependencies
        $this->database = $database;

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

        // Init events
        $this->gtk->connect(
            'configure-event',
            function(
                \GtkWindow $window,
                // \GdkEvent $event
            ) {
                // @TODO render data wordwrap by $window->get_size()
            }
        );
    }
}