<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Base;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Go;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\History;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Request;

class Navbar
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Page $page;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Base $base;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Go $go;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\History $history;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Request $request;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page $page
    ) {
        // Init dependencies
        $this->page = $page;

        // Init navbar
        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_margin_top(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_spacing(
            $this->_margin
        );

        // Append base button
        $this->base = new Base(
            $this
        );

        $this->gtk->add(
            $this->base->gtk
        );

        // Append history buttons group
        $this->history = new History(
            $this
        );

        $this->gtk->add(
            $this->history->gtk
        );

        // Append request entry, fill empty space
        $this->request = new Request(
            $this
        );

        $this->gtk->pack_start(
            $this->request->gtk,
            true,
            true,
            0
        );

        // Append go button
        $this->go = new Go(
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