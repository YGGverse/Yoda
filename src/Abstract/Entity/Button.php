<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Button
{
    public \GtkButton $gtk;

    public const SENSITIVE = false;
    public const IMAGE = null;
    public const LABEL = '';
    public const TOOLTIP = '';

    public function __construct()
    {
        $this->gtk = new \GtkButton;

        if (\GtkIconTheme::get_default()->has_icon($this::IMAGE))
        {
            $this->gtk->set_image(
                \GtkImage::new_from_icon_name(
                    $this::IMAGE,
                    \GtkIconSize::BUTTON
                )
            );
        }

        else
        {
            $this->gtk->set_label(
                _($this::LABEL)
            );
        }

        $this->gtk->set_sensitive(
            _($this::TOOLTIP)
        );

        $this->gtk->set_sensitive(
            $this::SENSITIVE
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