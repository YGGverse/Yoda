<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address;

class Statusbar
{
    public \GtkLabel $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address $address;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address $address
    ) {
        $this->address = $address;

        $this->gtk = new \GtkLabel;

        $this->gtk->set_line_wrap(
            true
        );

        $this->gtk->set_xalign(
            0
        );

        $this->gtk->set_yalign(
            0
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
    }
}