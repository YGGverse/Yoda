<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Content;

class Plain
{
    public \GtkLabel $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address\Content $content;

    // Defaults
    private string $_value = '';

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address\Content $content
    ) {
        $this->content = $content;

        $this->gtk = new \GtkLabel(
            $this->_value
        );

        $this->gtk->set_use_markup(
            false
        );

        $this->gtk->set_selectable(
            true
        );

        $this->gtk->set_line_wrap(
            true
        );

        $this->gtk->set_xalign(
            0
        );

        $this->gtk->set_yalign(
            0
        );
    }

    public function setValue(
        string $value
    ): void
    {
        $this->gtk->set_text(
            $value
        );
    }
}