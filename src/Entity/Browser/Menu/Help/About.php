<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Help;

use \Gtk;
use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu\Help;

class About
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Help $help;

    // Defaults
    public const LABEL = 'About';

    public function __construct(
        Help $help
    ) {
        // Init dependencies
        $this->help = $help;

        // Init menu item
        $this->gtk = GtkMenuItem::new_with_label(
            $this::LABEL
        );

        // Render
        $this->gtk->show();

        // Int events
        $this->gtk->connect(
            'activate',
            function()
            {
                // @TODO
            }
        );
    }
}