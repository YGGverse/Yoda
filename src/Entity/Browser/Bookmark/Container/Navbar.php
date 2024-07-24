<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Bookmark\Container;

use \GtkBox;
use \GtkOrientation;

use \Yggverse\Yoda\Entity\Browser\Bookmark\Container;

class Navbar
{
    public GtkBox $gtk;

    // Dependencies
    public Container $container;

    // Requirements
    public Navbar\Delete $delete;
    public Navbar\Filter $filter;
    public Navbar\Open $open;
    public Navbar\Search $search;

    // Defaults
    public const MARGIN = 8;
    public const SPACING = 8;

    public function __construct(
        Container $container
    ) {
        // Init dependency
        $this->container = $container;

        // Init container
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
            $this::SPACING
        );

        // Init open button
        $this->open = new Navbar\Open(
            $this
        );

        $this->gtk->add(
            $this->open->gtk
        );

        // Init delete button
        $this->delete = new Navbar\Delete(
            $this
        );

        $this->gtk->add(
            $this->delete->gtk
        );

        // Init filter entry
        $this->filter = new Navbar\Filter(
            $this
        );

        $this->gtk->pack_start(
            $this->filter->gtk,
            true,
            true,
            0
        );

        // Render
        $this->gtk->show();
    }

    public function refresh(): void
    {
        $this->delete->refresh();
        $this->open->refresh();
    }
}