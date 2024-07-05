<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \Yggverse\Yoda\Entity\Browser\Container\Tab;

class Container
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser $browser;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Tab $tab;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser $browser
    ) {
        // Init dependency
        $this->browser = $browser;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        // Init tab
        $this->tab = new Tab(
            $this
        );

        $this->gtk->pack_start(
            $this->tab->gtk,
            true,
            true,
            0
        );
    }

    public function refresh()
    {
        // @TODO
    }
}