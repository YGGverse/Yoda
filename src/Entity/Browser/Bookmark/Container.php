<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Bookmark;

use \GtkBox;
use \GtkOrientation;

use \Yggverse\Yoda\Entity\Browser\Bookmark;

class Container
{
    // GTK
    public GtkBox $gtk;

    // Dependencies
    public Bookmark $bookmark;

    // Requirements
    public Container\Navbar $navbar;
    public Container\Content $content;

    public function __construct(
        Bookmark $bookmark
    ) {
        // Init dependency
        $this->bookmark = $bookmark;

        // Init container
        $this->gtk = new GtkBox(
            GtkOrientation::VERTICAL
        );

        // Init navbar
        $this->navbar = new Container\Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        // Init content
        $this->content = new Container\Content(
            $this
        );

        $this->gtk->pack_start(
            $this->content->gtk,
            true,
            true,
            0
        );

        // Render
        $this->gtk->show();
    }

    public function refresh()
    {
        $this->navbar->refresh();
        $this->content->refresh();
    }
}