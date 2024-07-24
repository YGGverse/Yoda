<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \GtkHeaderBar;

use \Yggverse\Yoda\Entity\Browser;

class Header
{
    // GTK
    public GtkHeaderBar $gtk;

    // Dependencies
    public Browser $browser;

    // Requirements
    public Header\Tray $tray;

    // Defaults
    public const ACTIONS = true;
    public const TITLE = 'Yoda';
    public const SUBTITLE = '';

    public function __construct(
        Browser $browser
    ) {
        // Init dependencies
        $this->browser = $browser;

        // Init header
        $this->gtk = new GtkHeaderBar;

        $this->gtk->set_show_close_button(
            $this::ACTIONS
        );

        $this->gtk->set_title(
            $this::TITLE
        );

        $this->gtk->set_subtitle(
            $this::SUBTITLE
        );

        // Init tray area
        $this->tray = new Header\Tray(
            $this
        );

        $this->gtk->add(
            $this->tray->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function setTitle(
        ?string $title = null,
        ?string $subtitle = null
    ): void
    {
        $this->gtk->set_title(
            is_null($title) ? $this::TITLE : sprintf(
                '%s - %s',
                trim(
                    $title
                ),
                $this::TITLE
            )
        );

        $this->setSubtitle(
            $subtitle
        );
    }

    public function setSubtitle(
        ?string $subtitle = null
    ): void
    {
        $this->gtk->set_subtitle(
            is_null($subtitle) ? $this::SUBTITLE : trim(
                $subtitle
            )
        );
    }
}