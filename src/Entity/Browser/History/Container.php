<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History;

use \Yggverse\Yoda\Entity\Browser\History\Container\Navbar;
use \Yggverse\Yoda\Entity\Browser\History\Container\Content;

class Container
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\History $history;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\History\Container\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Browser\History\Container\Content $content;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\History $history
    ) {
        // Init dependency
        $this->history = $history;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        // Init navbar
        $this->navbar = new Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        // Init content
        $this->content = new Content(
            $this
        );

        $this->gtk->pack_start(
            $this->content->gtk,
            true,
            true,
            0
        );
    }

    public function refresh()
    {
        $this->navbar->refresh();
        $this->content->refresh();
    }
}