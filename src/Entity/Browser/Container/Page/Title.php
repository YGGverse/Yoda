<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

class Title
{
    public \GtkLabel $gtk;

    // Extras
    public ?string $subtitle = null;

    // Dependencies
    public Page $page;

    // Defaults
    private int $_ellipsize = 3;
    private int $_length = 16;
    private string $_value = 'New page';
    private string $_subtitle = '';
    private string $_tooltip = '';

    public function __construct(
        Page $page,
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
            is_null($value) ? _($this->_value) : trim(
                $value
            )
        );
    }

    public function setSubtitle(
        ?string $subtitle = null
    ): void
    {
        $this->subtitle = is_null($subtitle) ? _($this->_subtitle) : strtolower(
            trim(
                $subtitle
            )
        );
    }

    public function setTooltip(
        ?string $tooltip = null
    ): void
    {
        $this->gtk->set_tooltip_text(
            is_null($tooltip) ? _($this->_tooltip) : trim(
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
        return $this->subtitle;
    }

    public function getTooltip(): ?string
    {
        return $this->gtk->get_tooltip();
    }
}