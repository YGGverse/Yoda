<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\File;

class Open
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Menu\File $file;

    // Defaults
    private string $_label = 'Open';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Menu\File $file
    ) {
        // Init dependencies
        $this->file = $file;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            $this->_label
        );

        // Render
        $this->gtk->show();
    }
}