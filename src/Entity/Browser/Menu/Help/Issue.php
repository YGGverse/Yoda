<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Help;

use \Gtk;
use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu\Help;

class Issue
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Help $help;

    // Defaults
    public const LABEL = 'Issue report';
    public const URL = 'https://github.com/YGGverse/Yoda/issues';

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
                Gtk::show_uri_on_window(
                    $this->help->menu->browser->gtk,
                    $this::URL
                );
            }
        );
    }
}