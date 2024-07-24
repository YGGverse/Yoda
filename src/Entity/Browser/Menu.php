<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \GtkMenu;
use \GtkSeparatorMenuItem;

use \Yggverse\Yoda\Entity\Browser;

class Menu
{
    public GtkMenu $gtk;

    // Dependencies
    public Browser $browser;

    // Requirements
    public Menu\File $file;
    public Menu\Tab $tab;
    public Menu\Bookmark $bookmark;
    public Menu\History $history;
    public Menu\Quit $quit;

    public function __construct(
        Browser $browser
    ) {
        // Init dependencies
        $this->browser = $browser;

        // Init menu
        $this->gtk = new GtkMenu;

        // Init file menu item
        $this->file = new Menu\File(
            $this
        );

        $this->gtk->append(
            $this->file->gtk
        );

        // Init tab menu item
        $this->tab = new Menu\Tab(
            $this
        );

        $this->gtk->append(
            $this->tab->gtk
        );

        // Init bookmark menu item
        $this->bookmark = new Menu\Bookmark(
            $this
        );

        $this->gtk->append(
            $this->bookmark->gtk
        );

        // Init history menu item
        $this->history = new Menu\History(
            $this
        );

        $this->gtk->append(
            $this->history->gtk
        );

        // Add separator
        $this->gtk->append(
            new GtkSeparatorMenuItem
        );

        // Init quit menu item
        $this->quit = new Menu\Quit(
            $this
        );

        $this->gtk->append(
            $this->quit->gtk
        );

        // Render
        $this->gtk->show();
    }
}