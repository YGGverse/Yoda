<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \Yggverse\Yoda\Entity\Browser\Menu;

class Tab
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public Menu $menu;

    // Requirements
    public Tab\Add $add;
    public Tab\Close $close;

    // Defaults
    public const LABEL = 'Tab';

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
        $tab = new \GtkMenu;

        // Init new tab menu item
        $this->add = new Tab\Add(
            $this
        );

        $tab->append(
            $this->add->gtk
        );

        // Init close tab menu item
        $this->close = new Tab\Close(
            $this
        );

        $tab->append(
            $this->close->gtk
        );

        $this->gtk->set_submenu(
            $tab
        );

        // Render
        $this->gtk->show();
    }
}