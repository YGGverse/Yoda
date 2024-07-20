<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Tab;

class Close
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Menu\Tab $tab;

    // Defaults
    private string $_label = 'Close';
    private string $_tooltip = 'Close active tab (double click on tab)';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Menu\Tab $tab
    ) {
        // Init dependencies
        $this->tab = $tab;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
        );

        $this->gtk->set_tooltip_text(
            $this->_tooltip
        );

        // Render
        $this->gtk->show();

        // Int events
        $this->gtk->connect(
            'activate',
            function()
            {
                $this->tab->menu->browser->container->tab->close();
            }
        );
    }
}