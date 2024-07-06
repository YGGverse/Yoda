<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu;

class Quit
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu $menu;

    // Defaults
    private string $_label = 'Quit';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu $menu
    ) {
        // Init dependencies
        $this->menu = $menu;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
        );

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