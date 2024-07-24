<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Content;

use \GtkViewport;

use \Yggverse\Yoda\Entity\Browser\History\Container\Content;

class Viewport
{
    // GTK
    public GtkViewport $gtk;

    // Dependencies
    public Content $content;

    public function __construct(
        Content $content
    ) {
        // Init dependencies
        $this->content = $content;

        // Init viewport
        $this->gtk = new GtkViewport;

        // Render
        $this->gtk->show();
    }
}