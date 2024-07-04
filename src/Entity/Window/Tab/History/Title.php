<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History;

class Title
{
    public \GtkLabel $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\History $history;

    // Defaults
    private int $_ellipsize = 0;
    private int $_length = 12;
    private string $_value = 'History';

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\History $history
    ) {
        $this->history = $history;

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