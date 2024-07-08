<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \Yggverse\Yoda\Entity\Browser\Menu\File\Open;

class File
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Menu $menu;

    // Defaults
    private string $_label = 'File';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Menu $menu
    ) {
        // Init dependencies
        $this->menu = $menu;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
        );

        // Init submenu container
        $file = new \GtkMenu;

        // Init new tab menu item
        $open = new Open(
            $this
        );

        $file->append(
            $open->gtk
        );

        $this->gtk->set_submenu(
            $file
        );

        // Render
        $this->gtk->show();
    }
}