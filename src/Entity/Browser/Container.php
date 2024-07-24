<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \GtkBox;
use \GtkOrientation;

use \Yggverse\Yoda\Entity\Browser;

class Container
{
    // GTK
    public GtkBox $gtk;

    // Dependencies
    public Browser $browser;

    // Requirements
    public Container\Tab $tab;

    public function __construct(
        Browser $browser
    ) {
        // Init dependency
        $this->browser = $browser;

        // Init container
        $this->gtk = new GtkBox(
            GtkOrientation::VERTICAL
        );

        // Init tab
        $this->tab = new Container\Tab(
            $this
        );

        $this->gtk->pack_start(
            $this->tab->gtk,
            true,
            true,
            0
        );

        // Render
        $this->gtk->show();
    }

    public function refresh()
    {
        // @TODO
    }
}