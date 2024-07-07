<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Content;

class Viewport
{
    public \GtkViewport $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\History\Container\Content $content;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\History\Container\Content $content
    ) {
        // Init dependencies
        $this->content = $content;

        // Init viewport
        $this->gtk = new \GtkViewport;

        // Render
        $this->gtk->show();
    }
}