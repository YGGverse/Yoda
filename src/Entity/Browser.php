<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity;

use \Gtk;
use \GtkWindow;
use \GtkWindowType;

use \Yggverse\Yoda\Entity\Browser\Header;
use \Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Model\Database;

class Browser
{
    // GTK
    public GtkWindow $gtk;

    // Dependencies
    public Database $database;

    // Requirements
    public Header $header;
    public Container $container;

    // Defaults
    public const WIDTH = 640;
    public const HEIGHT = 640;
    public const MAXIMIZE = true;
    public const DEBUG = false;

    public function __construct(
        Database $database
    ) {
        // Init dependencies
        $this->database = $database;

        // Init window
        $this->gtk = new GtkWindow(
            GtkWindowType::TOPLEVEL
        );

        $this->gtk->set_interactive_debugging(
            $this::DEBUG
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
            'destroy',
            function()
            {
                // Save session data
                $pid = pcntl_fork();

                if ($pid === 0)
                {
                    // Reset previous records
                    $this->database->session->clean();

                    foreach ($this->container->tab->pages as $page)
                    {
                        // Save page session data
                        $this->database->session->add(
                            $page->navbar->request->getValue()
                        );

                        // Cache connection pool data
                        if ($page->connection)
                        {
                            $this->database->cache->renew(
                                $page->navbar->request->getValue(),
                                $page->connection->getMime(),
                                $page->connection->getTitle(),
                                $page->connection->getSubtitle(),
                                $page->connection->getTooltip(),
                                $page->connection->getData()
                            );
                        }
                    }

                    exit;
                }

                // Exit application
                Gtk::main_quit();
            }
        );
    }
}