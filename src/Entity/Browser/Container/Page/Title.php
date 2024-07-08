<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Gtk\Browser\Container\Page\Title\Label;

class Title
{
    public Label $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Page $page;

    // Defaults
    private int $_ellipsize = 3;
    private int $_length = 16;
    private ?string $_value = 'New page';
    private ?string $_subtitle = null;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page $page,
    ) {
        // Init dependencies
        $this->page = $page;

        // Init container
        $this->gtk = new Label(
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
        ?string $value = null,
        ?string $subtitle = null
    ): void
    {
        $this->gtk->set_text(
            is_null($value) ? $this->_value : trim(
                $value
            )
        );

        $this->setSubtitle(
            $subtitle
        );
    }

    public function setSubtitle(
        ?string $subtitle = null
    ): void
    {
        $this->gtk->set_subtitle(
            is_null($subtitle) ? $this->_subtitle : strtolower(
                trim(
                    $subtitle
                )
            )
        );
    }

    public function getValue(): ?string
    {
        return $this->gtk->get_text();
    }

    public function getSubtitle(): ?string
    {
        return $this->gtk->get_subtitle();
    }
}