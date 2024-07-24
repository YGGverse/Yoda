<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Tab;

use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu\Tab;

class Add
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Tab $tab;

    // Defaults
    public const LABEL = 'Add';
    public const TOOLTIP = 'New tab';

    public function __construct(
        Tab $tab
    ) {
        // Init dependencies
        $this->tab = $tab;

        // Init menu item
        $this->gtk = GtkMenuItem::new_with_label(
            _($this::LABEL)
        );

        $this->gtk->set_tooltip_text(
            _($this::TOOLTIP)
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate',
            function()
            {
                $this->tab->menu->browser->container->tab->append(
                    null,
                    false
                );
            }
        );
    }
}