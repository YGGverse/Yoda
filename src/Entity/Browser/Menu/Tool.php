<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \GtkMenu;
use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu;

class Tool
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Menu $menu;

    // Requirements
    public Tool\Debug $debug;

    // Defaults
    public const LABEL = 'Tool';

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
        $tool = new GtkMenu;

        // Init debug menu item
        $this->debug = new Tool\Debug(
            $this
        );

        $tool->append(
            $this->debug->gtk
        );

        $this->gtk->set_submenu(
            $tool
        );

        // Render
        $this->gtk->show();
    }
}