<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page as Page;

class Navbar
{
    public \GtkBox $gtk;

    // Dependencies
    public Page $page;

    // Requirements
    public Navbar\Base $base;
    public Navbar\Go $go;
    public Navbar\History $history;
    public Navbar\Request $request;

    // Defaults
    public const MARGIN = 8;

    public function __construct(
        Page $page
    ) {
        // Init dependencies
        $this->page = $page;

        // Init navbar
        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_margin_top(
            $this::MARGIN
        );

        $this->gtk->set_margin_bottom(
            $this::MARGIN
        );

        $this->gtk->set_margin_start(
            $this::MARGIN
        );

        $this->gtk->set_margin_end(
            $this::MARGIN
        );

        $this->gtk->set_spacing(
            $this::MARGIN
        );

        // Append base button
        $this->base = new Navbar\Base(
            $this
        );

        $this->gtk->add(
            $this->base->gtk
        );

        // Append history buttons group
        $this->history = new Navbar\History(
            $this
        );

        $this->gtk->add(
            $this->history->gtk
        );

        // Append request entry, fill empty space
        $this->request = new Navbar\Request(
            $this
        );

        $this->gtk->pack_start(
            $this->request->gtk,
            true,
            true,
            0
        );

        // Append go button
        $this->go = new Navbar\Go(
            $this
        );

        $this->gtk->add(
            $this->go->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function refresh()
    {
        $this->base->refresh();
        $this->go->refresh();
        $this->history->refresh();
    }
}