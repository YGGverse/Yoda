<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

class Quit
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Menu $menu;

    // Defaults
    private string $_label = 'Quit';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Menu $menu
    ) {
        // Init dependencies
        $this->menu = $menu;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
        );

        // Render
        $this->gtk->show();

        // Int events
        $this->gtk->connect(
            'activate',
            function()
            {
                \Gtk::main_quit();
            }
        );
    }
}