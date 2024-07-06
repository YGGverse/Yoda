<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header\Navigation;

use \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu\History;
use \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu\Quit;

class Menu
{
    public \GtkMenu $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Header\Navigation $navigation;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu\History $history;
    public \Yggverse\Yoda\Entity\Browser\Header\Navigation\Menu\Quit $quit;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Header\Navigation $navigation
    ) {
        // Init dependencies
        $this->navigation = $navigation;

        // Init menu
        $this->gtk = new \GtkMenu;

        // Init history
        $this->history = new History(
            $this
        );

        $this->gtk->append(
            $this->history->gtk
        );

        // Init quit
        $this->quit = new Quit(
            $this
        );

        $this->gtk->append(
            $this->quit->gtk
        );

        // Render
        $this->gtk->show_all();
    }
}