<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address;

class Title
{
    public \GtkLabel $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address $address;

    // Defaults
    private int $_ellipsize = 3;
    private int $_length = 12;
    private string $_text = 'New address';

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address $address,
    ) {
        $this->address = $address;

        $this->gtk = new \GtkLabel(
            $this->_text
        );

        $this->gtk->set_width_chars(
            $this->_length
        );

        $this->gtk->set_ellipsize(
            $this->_ellipsize
        );
    }

    public function setText(
        ?string $text = null
    ): void
    {
        $this->gtk->set_text(
            is_null($text) ? $this->_text : trim(
                $text
            )
        );
    }
}