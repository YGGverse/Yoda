<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header;

use \Yggverse\Yoda\Entity\Browser\Menu;

class Navigation
{
    public \GtkMenuButton $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Header $header;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Menu $menu;

    // Defaults
    private string $_tooltip = 'Navigation';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Header $header
    ) {
        // Init dependencies
        $this->header = $header;

        // Init navigation container
        $this->gtk = new \GtkMenuButton;

        $this->gtk->set_tooltip_text(
            $this->_tooltip
        );

        // Init menu
        $this->menu = new Menu(
            $this->header->browser
        );

        $this->gtk->set_popup(
            $this->menu->gtk
        );

        // Render
        $this->gtk->show();
    }
}