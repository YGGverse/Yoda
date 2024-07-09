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
    private ?string $_tooltip = null;

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

    public function set(
        ?string $value = null,
        ?string $subtitle = null,
        ?string $tooltip = null
    ): void
    {
        $this->setValue(
            $value
        );

        $this->setSubtitle(
            $subtitle
        );

        $this->setTooltip(
            is_null($tooltip) ? (mb_strlen(strval($value)) > $this->_length ? $value : null)
                              : $tooltip
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

    public function setTooltip(
        ?string $tooltip = null
    ): void
    {
        $this->gtk->set_tooltip_text(
            is_null($tooltip) ? $this->_tooltip : trim(
                $tooltip
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

    public function getTooltip(): ?string
    {
        return $this->gtk->get_tooltip();
    }
}