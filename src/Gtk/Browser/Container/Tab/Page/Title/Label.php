<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Gtk\Browser\Container\Tab\Page\Title;

class Label extends \GtkLabel
{
    private string $_subtitle = '';

    public function set_subtitle(
        string $value
    ): void
    {
        $this->_subtitle = $value;
    }

    public function get_subtitle(): string
    {
        return $this->_subtitle;
    }
}