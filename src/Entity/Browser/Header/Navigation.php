<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header;

use \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu;

class Navigation
{
    public \GtkMenuButton $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Header $header;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu $menu;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Header $header
    ) {
        // Init dependencies
        $this->header = $header;

        // Init navigation container
        $this->gtk = new \GtkMenuButton;

        // Init menu
        $this->menu = new Menu(
            $this
        );

        $this->gtk->set_popup(
            $this->menu->gtk
        );
    }
}