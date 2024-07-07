<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Button
{
    public \GtkButton $gtk;

    protected bool   $_sensitive = false;
    protected string $_label = 'Button';

    public function __construct()
    {
        $this->gtk = new \GtkButton;

        $this->gtk->set_sensitive(
            $this->_sensitive
        );

        $this->gtk->set_label(
            $this->_label
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