<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Data;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Viewport;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Page $page;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Data $data;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Viewport $viewport;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page $page
    ) {
        $this->page = $page;

        // Init container
        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        // Init viewport
        // to integrate scrolled window features for data label
        $this->viewport = new Viewport(
            $this
        );

        // Init data label
        $this->data = new Data(
            $this
        );

        $this->viewport->gtk->add(
            $this->data->gtk
        );

        $this->gtk->add(
            $this->viewport->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function refresh()
    {
        // @TODO
    }

    public function setGemtext(
        ?string $data = null,
        ?string &$title = null
    ): void
    {
        $this->data->setGemtext(
            $data,
            $title
        );
    }

    public function setPlain(
        ?string $data = null
    ): void
    {
        $this->data->setPlain(
            $data
        );
    }
}