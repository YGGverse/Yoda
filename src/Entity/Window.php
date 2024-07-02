<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity;

use \Yggverse\Yoda\Entity\Window\Header;
use \Yggverse\Yoda\Entity\Window\Tab;

class Window
{
    public \GtkWindow $gtk;

    public \Yggverse\Yoda\Model\Database $database;

    public \Yggverse\Yoda\Entity\Window\Header $header;
    public \Yggverse\Yoda\Entity\Window\Tab $tab;

    // Defaults
    private int $_width  = 640;
    private int $_height = 480;

    public function __construct(
        \Yggverse\Yoda\Model\Database $database
    ) {
        $this->database = $database;

        $this->gtk = new \GtkWindow;

        $this->gtk->set_size_request(
            $this->_width,
            $this->_height
        );

        $this->header = new Header(
            $this
        );

        $this->gtk->set_titlebar(
            $this->header->gtk
        );

        $this->tab = new Tab(
            $this
        );

        $this->gtk->add(
            $this->tab->gtk
        );

        $this->gtk->show_all();
    }
}