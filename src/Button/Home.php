<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Button;

class Home
{
    public \GtkButton $button;

    public function __construct(
        ?string $label = 'Home'
    ) {
        $this->button = \GtkButton::new_with_label(
            $label
        );
    }
}