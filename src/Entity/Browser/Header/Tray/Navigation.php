<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header\Tray;

use \Yggverse\Yoda\Entity\Browser\Header\Tray;
use \Yggverse\Yoda\Entity\Browser\Menu;

class Navigation
{
    public \GtkMenuButton $gtk;

    // Dependencies
    public Tray $tray;

    // Requirements
    public Menu $menu;

    // Defaults
    public const TOOLTIP = 'Navigation';

    public function __construct(
        Tray $tray
    ) {
        // Init dependencies
        $this->tray = $tray;

        // Init navigation container
        $this->gtk = new \GtkMenuButton;

        $this->gtk->set_tooltip_text(
            _($this::TOOLTIP)
        );

        // Init menu
        $this->menu = new Menu(
            $this->tray->header->browser
        );

        $this->gtk->set_popup(
            $this->menu->gtk
        );

        // Render
        $this->gtk->show();
    }
}