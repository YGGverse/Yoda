<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \Yggverse\Yoda\Entity\Browser\Menu\File;
use \Yggverse\Yoda\Entity\Browser\Menu\Tab;
use \Yggverse\Yoda\Entity\Browser\Menu\History;
use \Yggverse\Yoda\Entity\Browser\Menu\Quit;

class Menu
{
    public \GtkMenu $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser $browser;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Menu\File $file;
    public \Yggverse\Yoda\Entity\Browser\Menu\Tab $tab;
    public \Yggverse\Yoda\Entity\Browser\Menu\History $history;
    public \Yggverse\Yoda\Entity\Browser\Menu\Quit $quit;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser $browser
    ) {
        // Init dependencies
        $this->browser = $browser;

        // Init menu
        $this->gtk = new \GtkMenu;

        // Init file menu item
        $this->file = new File(
            $this
        );

        $this->gtk->append(
            $this->file->gtk
        );

        // Init tab menu item
        $this->tab = new Tab(
            $this
        );

        $this->gtk->append(
            $this->tab->gtk
        );

        // Init history menu item
        $this->history = new History(
            $this
        );

        $this->gtk->append(
            $this->history->gtk
        );

        // Add separator
        $this->gtk->append(
            new \GtkSeparatorMenuItem
        );

        // Init quit menu item
        $this->quit = new Quit(
            $this
        );

        $this->gtk->append(
            $this->quit->gtk
        );

        // Render
        $this->gtk->show();
    }
}