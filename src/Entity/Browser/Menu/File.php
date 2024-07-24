<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \GtkMenu;
use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu;

class File
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Menu $menu;

    // Defaults
    public const LABEL = 'File';

    public function __construct(
        Menu $menu
    ) {
        // Init dependencies
        $this->menu = $menu;

        // Init menu item
        $this->gtk = GtkMenuItem::new_with_label(
            $this::LABEL
        );

        // Init submenu container
        $file = new GtkMenu;

        // Init tab menu items
        $open = new File\Open(
            $this
        );

        $file->append(
            $open->gtk
        );

        $save = new File\Save(
            $this
        );

        $file->append(
            $save->gtk
        );

        $this->gtk->set_submenu(
            $file
        );

        // Render
        $this->gtk->show();
    }
}