<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Label;

class Content
{
    public \GtkLabel $label;

    public function __construct(string $value = '')
    {
        $this->label = new \GtkLabel(
            $value
        );

        $this->label->set_use_markup(
            false
        );

        $this->label->set_selectable(
            true
        );

        $this->label->set_line_wrap(
            true
        );

        $this->label->set_xalign(
            0
        );

        $this->label->set_yalign(
            0
        );
    }
}