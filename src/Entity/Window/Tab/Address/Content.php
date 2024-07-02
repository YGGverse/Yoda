<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address;

use \Yggverse\Yoda\Entity\Window\Tab\Address\Content\Gemtext;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Content\Plain;

class Content
{
    public \GtkScrolledWindow $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address $address;

    public Gemtext | Plain $data;

    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address $address
    ) {
        $this->address = $address;

        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->data = new Gemtext(
            $this
        );

        $this->gtk->add(
            $this->data->gtk
        );

        $this->gtk->show_all();
    }
}