<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Button;

class Go
{
    public \GtkButton $button;

    public function __construct(
        ?string $label = 'Go'
    ) {
        $this->button = \GtkButton::new_with_label(
            $label
        );
    }
}