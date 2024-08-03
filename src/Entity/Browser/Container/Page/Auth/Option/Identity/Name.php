<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Auth\Option\Identity;

use \GtkEntry;

class Name
{
    // GTK
    public GtkEntry $gtk;

    // Dependencies
    public Identity $identity;

    // Defaults
    public const ALIGNMENT = 0.5;
    public const MARGIN = 8;
    public const PLACEHOLDER = 'Local name (optional)';

    public function __construct(
        Identity $identity
    ) {
        // Init dependencies
        $this->identity = $identity;

        // Init GTK
        $this->gtk = new GtkEntry;

        $this->_name->set_alignment(
            $this::ALIGNMENT
        );

        $this->_name->set_placeholder_text(
            _($this::PLACEHOLDER)
        );

        $this->_name->set_margin_start(
            $this::MARGIN
        );

        $this->_name->set_margin_end(
            $this::MARGIN
        );

        $this->_name->set_margin_bottom(
            $this::MARGIN
        );

        $this->gtk->show();
    }
}