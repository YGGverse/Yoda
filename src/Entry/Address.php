<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entry;

class Address
{
    public \GtkEntry $entry;

    public function __construct(
        ?string $text = null,
        ?string $placeholder = 'URL or any search term...'
    ) {
        $this->entry = new \GtkEntry();

        $this->entry->set_text(
            $text
        );

        $this->entry->set_placeholder_text(
            $placeholder
        );

        $this->entry->set_max_length(
            1024
        );
    }
}