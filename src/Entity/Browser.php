<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity;

use \Yggverse\Yoda\Entity\Browser\Header;
use \Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Model\Database;

class Browser
{
    public \GtkWindow $gtk;

    // Dependencies
    public Database $database;

    // Requirements
    public Header $header;
    public Container $container;

    // Defaults
    public const WIDTH = 640;
    public const HEIGHT = 640;
    public const MAXIMIZE = true;

    public function __construct(
        Database $database
    ) {
        // Init dependencies
        $this->database = $database;

        // Init window
        $this->gtk = new \GtkWindow;

        $this->gtk->set_size_request(
            $this::WIDTH,
            $this::HEIGHT
        );

        if ($this::MAXIMIZE)
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