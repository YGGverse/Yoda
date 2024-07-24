<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header\Tray;

use \GtkButton;
use \GtkIconSize;
use \GtkIconTheme;
use \GtkImage;

use \Yggverse\Yoda\Entity\Browser\Header\Tray;

class Tab
{
    // GTK
    public GtkButton $gtk;

    // Dependencies
    public Tray $tray;

    // Defaults
    public const LABEL = '+';
    public const IMAGE = 'tab-new-symbolic';
    public const TOOLTIP = 'New tab';

    public function __construct(
        Tray $tray
    ) {
        // Init dependency
        $this->tray = $tray;

        // Init GTK
        $this->gtk = new GtkButton;

        if (GtkIconTheme::get_default()->has_icon($this::IMAGE))
        {
            $this->gtk->set_image(
                GtkImage::new_from_icon_name(
                    $this::IMAGE,
                    GtkIconSize::BUTTON
                )
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

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'clicked',
            function(
                GtkButton $entity
            ) {
                $this->tray->header->browser->container->tab->append(
                    null,
                    false
                );
            }
        );
    }
}