<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entry;

class Address
{
    public \GtkEntry $entry;

    public function __construct(
        ?string $value = null
    ) {
        $this->entry = new \GtkEntry();

        $this->entry->set_text(
            $value
        );
    }
}