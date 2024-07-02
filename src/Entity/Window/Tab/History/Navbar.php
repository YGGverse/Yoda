<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History;

use \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Delete;
use \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Filter;
use \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Open;
use \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Search;

class Navbar
{
    public \GtkBox $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\History $history;

    public \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Delete $delete;
    public \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Filter $filter;
    public \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Open $open;
    public \Yggverse\Yoda\Entity\Window\Tab\History\Navbar\Search $search;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\History $history
    ) {
        $this->history = $history;

        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->setMargin(
            $this->_margin
        );

        $this->open = new Open(
            $this
        );

        $this->gtk->add(
            $this->open->gtk
        );

        $this->delete = new Delete(
            $this
        );

        $this->gtk->add(
            $this->delete->gtk
        );

        $this->filter = new Filter(
            $this
        );

        $this->gtk->pack_start(
            $this->filter->gtk,
            true,
            true,
            0
        );

        $this->search = new Search(
            $this
        );

        $this->gtk->add(
            $this->search->gtk
        );
    }

    public function setMargin(
        ?int $value = null
    ): void
    {
        $this->gtk->set_margin_top(
            $margin ?? $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $margin ?? $this->_margin
        );

        $this->gtk->set_margin_start(
            $margin ?? $this->_margin
        );

        $this->gtk->set_margin_end(
            $margin ?? $this->_margin
        );

        $this->gtk->set_spacing(
            $margin ?? $this->_margin
        );
    }
}