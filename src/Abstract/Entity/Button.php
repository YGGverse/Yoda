<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Button
{
    public \GtkButton $gtk;

    public const SENSITIVE = false;
    public const LABEL = 'Button';

    public function __construct()
    {
        $this->gtk = new \GtkButton;

        $this->gtk->set_sensitive(
            $this::SENSITIVE
        );

        $this->gtk->set_label(
            _($this::LABEL)
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'clicked',
            function(
                \GtkButton $entity
            ) {
                $this->_onClick(
                    $entity
                );
            }
        );
    }

    abstract protected function _onClick(
        \GtkButton $entity
    ): void;
}