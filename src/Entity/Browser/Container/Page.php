<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Title;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Progressbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Response;

class Page
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container $container;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Title $title;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Progressbar $progressbar;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content $content;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Response $response;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container $container
    ) {
        // Init dependencies
        $this->container = $container;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        // Init title
        $this->title = new Title(
            $this
        );

        // Init navbar
        $this->navbar = new Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        // Init content
        $this->content = new Content(
            $this
        );

        $this->gtk->pack_start(
            $this->content->gtk,
            true,
            true,
            0
        );

        // Init progress bar
        $this->progressbar = new Progressbar(
            $this
        );

        $this->gtk->add(
            $this->progressbar->gtk
        );

        // Init response bar
        $this->response = new Response(
            $this
        );

        $this->gtk->pack_end(
            $this->response->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function refresh(): void
    {
        $this->navbar->refresh();
        $this->content->refresh();
    }

    public function open(
        ?string $request = null,
        bool $history = true
    ): void
    {
        $this->navbar->request->setValue(
            $request
        );

        $this->update(
            $history
        );
    }

    public function update(
        bool $history = true
    ): void
    {
        // Show progressbar
        $this->progressbar->infinitive();

        // Update content entity
        $this->content->update(
            $history
        );

        // Hide progressbar
        $this->progressbar->hide();
    }
}