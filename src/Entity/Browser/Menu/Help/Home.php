<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Help;

use \Gtk;
use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu\Help;

class Home
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Help $help;

    // Defaults
    public const LABEL = 'Home page';
    public const URL = 'gemini://yggverse.cities.yesterweb.org';

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
                $this->help->menu->browser->container->tab->append(
                    self::URL
                );
            }
        );
    }
}