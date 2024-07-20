<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Tab;

use \Yggverse\Yoda\Entity\Browser\Menu\Tab;

class Add
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public Tab $tab;

    // Defaults
    private string $_label = 'Add';

    public function __construct(
        Tab $tab
    ) {
        // Init dependencies
        $this->tab = $tab;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
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