<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \Yggverse\Yoda\Entity\Browser\Menu\Tab\Add;
use \Yggverse\Yoda\Entity\Browser\Menu\Tab\Close;

class Tab
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Menu $menu;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Menu\Tab\Add $add;
    public \Yggverse\Yoda\Entity\Browser\Menu\Tab\Close $close;

    // Defaults
    private string $_label = 'Tab';

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
        $tab = new \GtkMenu;

        // Init new tab menu item
        $this->add = new Add(
            $this
        );

        $tab->append(
            $this->add->gtk
        );

        // Init close tab menu item
        $this->close = new Close(
            $this
        );

        $tab->append(
            $this->close->gtk
        );

        $this->gtk->set_submenu(
            $tab
        );
    }
}