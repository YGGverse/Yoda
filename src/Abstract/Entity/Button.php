<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Button
{
    public \GtkButton $gtk;

    public const SENSITIVE = false;
    public const LABEL = 'Button';
    public const IMAGE = null;

    public function __construct()
    {
        $this->gtk = new \GtkButton;

        if ($this::IMAGE && \GtkIconTheme::get_default()->has_icon($this::IMAGE))
        {
            $this->gtk->set_image(
                \GtkImage::new_from_icon_name(
                    $this::IMAGE,
                    \GtkIconSize::BUTTON
                )
            );
        }

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