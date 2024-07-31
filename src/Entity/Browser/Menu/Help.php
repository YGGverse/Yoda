<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \GtkMenu;
use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu;

class Help
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Menu $menu;

    // Requirements
    public Help\About $about;
    public Help\Debug $debug;
    public Help\Home $home;
    public Help\Issue $issue;

    // Defaults
    public const LABEL = 'Help';

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
        $help = new GtkMenu;

        // Init about menu item
        $this->about = new Help\About(
            $this
        );

        $help->append(
            $this->about->gtk
        );

        // Init home menu item
        $this->home = new Help\Home(
            $this
        );

        $help->append(
            $this->home->gtk
        );

        // Init issue menu item
        $this->issue = new Help\Issue(
            $this
        );

        $help->append(
            $this->issue->gtk
        );

        // Set submenu
        $this->gtk->set_submenu(
            $help
        );

        // Render
        $this->gtk->show();
    }
}