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
        // Init button
        $this->gtk = new \GtkButton;

        if ($this::IMAGE)
        {
            $this->setImage(
                $this::IMAGE
            );
        }

        else
        {
            $this->gtk->set_label(
                _($this::LABEL)
            );
        }

        $this->gtk->set_tooltip_text(
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

    public function setImage(
        ?string $image = null,
        int $size = \GtkIconSize::BUTTON
    ): void
    {
        if (\GtkIconTheme::get_default()->has_icon($image))
        {
            $this->gtk->set_image(
                \GtkImage::new_from_icon_name(
                    $image,
                    $size
                )
            );

        } else throw new \Exception;
    }
}