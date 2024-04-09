<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Button;

class Forward
{
    public \GtkButton $button;

    public function __construct(
        ?string $label = 'Forward'
    ) {
        $this->button = \GtkButton::new_with_label(
            $label
        );
    }
}