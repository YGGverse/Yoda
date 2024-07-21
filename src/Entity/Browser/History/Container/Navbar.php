<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container;

use \Yggverse\Yoda\Entity\Browser\History\Container;

use \Yggverse\Yoda\Entity\Browser\History\Container\Navbar\Delete;
use \Yggverse\Yoda\Entity\Browser\History\Container\Navbar\Filter;
use \Yggverse\Yoda\Entity\Browser\History\Container\Navbar\Open;
use \Yggverse\Yoda\Entity\Browser\History\Container\Navbar\Search;

class Navbar
{
    public \GtkBox $gtk;

    // Dependencies
    public Container $container;

    // Requirements
    public Delete $delete;
    public Filter $filter;
    public Open $open;
    public Search $search;

    // Defaults
    public const MARGIN = 8;
    public const SPACING = 8;

    public function __construct(
        Container $container
    ) {
        // Init dependency
        $this->container = $container;

        // Init container
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
            $this::SPACING
        );

        // Init open button
        $this->open = new Open(
            $this
        );

        $this->gtk->add(
            $this->open->gtk
        );

        // Init delete button
        $this->delete = new Delete(
            $this
        );

        $this->gtk->add(
            $this->delete->gtk
        );

        // Init filter entry
        $this->filter = new Filter(
            $this
        );

        $this->gtk->pack_start(
            $this->filter->gtk,
            true,
            true,
            0
        );

        // Init search button
        $this->search = new Search(
            $this
        );

        $this->gtk->add(
            $this->search->gtk
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