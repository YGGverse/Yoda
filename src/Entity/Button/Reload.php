<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Button;

class Reload
{
    public \GtkButton $button;

    public function __construct(
        ?string $label = 'Reload'
    ) {
        $this->button = \GtkButton::new_with_label(
            $label
        );
    }
}