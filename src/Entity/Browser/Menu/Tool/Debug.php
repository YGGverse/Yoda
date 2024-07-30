<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Tool;

use \GtkMenuItem;

use \Yggverse\Yoda\Entity\Browser\Menu\Tool;

class Debug
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Tool $tool;

    // Defaults
    public const LABEL = 'Debug';

    public function __construct(
        Tool $tool
    ) {
        // Init dependencies
        $this->tool = $tool;

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
                $this->tool->menu->browser->gtk->set_interactive_debugging(
                    true
                );
            }
        );
    }
}