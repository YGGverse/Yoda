<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Label;

class Tray
{
    public \GtkLabel $label;

    public function __construct(string $value = '')
    {
        $this->label = new \GtkLabel(
            $value
        );

        $this->label->set_use_markup(
            true
        );

        $this->label->set_selectable(
            false
        );

        $this->label->set_xalign(
            0
        );

        $this->label->set_yalign(
            0
        );
    }
}