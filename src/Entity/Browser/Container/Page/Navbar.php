<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \GtkBox;
use \GtkOrientation;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

class Navbar
{
    // GTK
    public GtkBox $gtk;

    // Dependencies
    public Page $page;

    // Requirements
    public Navbar\Base $base;
    public Navbar\Bookmark $bookmark;
    public Navbar\History $history;
    public Navbar\Request $request;
    public Navbar\Update $update;

    // Defaults
    public const MARGIN = 8;

    public function __construct(
        Page $page
    ) {
        // Init dependencies
        $this->page = $page;

        // Init navbar
        $this->gtk = new GtkBox(
            GtkOrientation::HORIZONTAL
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

        // Append update button
        $this->update = new Navbar\Update(
            $this
        );

        $this->gtk->add(
            $this->update->gtk
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

        // Append bookmark button
        $this->bookmark = new Navbar\Bookmark(
            $this
        );

        $this->gtk->add(
            $this->bookmark->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function refresh()
    {
        $this->base->refresh();
        $this->bookmark->refresh();
        $this->history->refresh();
        $this->update->refresh();
    }
}