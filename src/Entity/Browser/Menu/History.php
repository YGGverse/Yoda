<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu;

use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu;

use \Yggverse\Yoda\Entity\Browser\History as Window;

class History
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Menu $menu;

    // Defaults
    public const LABEL = 'History';

    public function __construct(
        Menu $menu
    ) {
        // Init dependencies
        $this->menu = $menu;

        // Init menu item
        $this->gtk = GtkMenuItem::new_with_label(
            $this::LABEL
        );

        // Render
        $this->gtk->show();

        // Int events
        $this->gtk->connect(
            'activate',
            function()
            {
                $history = new Window(
                    $this->menu->browser
                );
            }
        );
    }
}