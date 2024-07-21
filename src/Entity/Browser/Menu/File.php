<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \Yggverse\Yoda\Entity\Browser\Menu;

use \Yggverse\Yoda\Entity\Browser\Menu\File\Open;
use \Yggverse\Yoda\Entity\Browser\Menu\File\Save;

class File
{
    public \GtkMenuItem $gtk;

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
        $this->gtk = \GtkMenuItem::new_with_label(
            $this::LABEL
        );

        // Init submenu container
        $file = new \GtkMenu;

        // Init tab menu items
        $open = new Open(
            $this
        );

        $file->append(
            $open->gtk
        );

        $save = new Save(
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