<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Button;

class Back
{
    public \GtkButton $button;

    public function __construct(
        ?string $label = 'Back'
    ) {
        $this->button = \GtkButton::new_with_label(
            $label
        );
    }
}