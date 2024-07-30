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
    public Help\Gemlog $gemlog;
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
        $tab = new GtkMenu;

        // Init about menu item
        $this->about = new Help\About(
            $this
        );

        $tab->append(
            $this->about->gtk
        );

        // Init gemlog menu item
        $this->gemlog = new Help\Gemlog(
            $this
        );

        $tab->append(
            $this->gemlog->gtk
        );

        // Init issue menu item
        $this->issue = new Help\Issue(
            $this
        );

        $tab->append(
            $this->issue->gtk
        );

        $this->gtk->set_submenu(
            $tab
        );

        // Render
        $this->gtk->show();
    }
}