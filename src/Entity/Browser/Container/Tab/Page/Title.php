<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page;

class Title
{
    public \GtkLabel $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page $page;

    // Defaults
    private int $_ellipsize = 3;
    private int $_length = 12;
    private string $_value = 'New page';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Tab\Page $page,
    ) {
        // Init dependencies
        $this->page = $page;

        // Init container
        $this->gtk = new \GtkLabel(
            $this->_value
        );

        $this->gtk->set_width_chars(
            $this->_length
        );

        $this->gtk->set_ellipsize(
            $this->_ellipsize
        );
    }

    public function setValue(
        ?string $value = null
    ): void
    {
        $this->gtk->set_text(
            is_null($value) ? $this->_value : trim(
                $value
            )
        );
    }
}