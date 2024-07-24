<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \GtkViewport;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;

class Viewport
{
    // GTK
    public GtkViewport $gtk;

    // Dependencies
    public Content $content;

    // Requirements
    private Gemtext | Plain | Image | null $_data = null;

    public function __construct(
        Content $content
    ) {
        // Init dependencies
        $this->content = $content;

        // Init viewport
        $this->gtk = new GtkViewport;
    }

    public function set(
        Gemtext | Plain | Image $data
    ): void
    {
        // Remove previous
        if ($this->_data)
        {
            $this->_data->gtk->destroy();
        }

        // Init current
        $this->_data = $data;

        $this->gtk->add(
            $data->gtk
        );

        // Render
        $this->gtk->show_all();
    }
}