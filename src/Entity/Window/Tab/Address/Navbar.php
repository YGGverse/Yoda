<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address;

use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\Base;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\Go;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\Request;

class Navbar
{
    public \GtkBox $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address $address;

    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\Base $base;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\Go $go;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History $history;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\Request $request;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address $address
    ) {
        $this->address = $address;

        // Init navbar area
        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->setMargins(
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
    }

    public function setMargins(
        ?int $value
    ): void
    {
        $this->gtk->set_margin_top(
            $value ?? $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $value ?? $this->_margin
        );

        $this->gtk->set_margin_start(
            $value ?? $this->_margin
        );

        $this->gtk->set_margin_end(
            $value ?? $this->_margin
        );

        $this->gtk->set_spacing(
            $value ?? $this->_margin
        );
    }
}