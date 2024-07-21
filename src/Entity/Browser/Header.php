<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \Yggverse\Yoda\Entity\Browser;

class Header
{
    public \GtkHeaderBar $gtk;

    // Dependencies
    public Browser $browser;

    // Requirements
    public Header\Tray $tray;

    // Defaults
    protected bool $_actions = true;
    protected string $_title = 'Yoda';
    protected string $_subtitle = '';

    public function __construct(
        Browser $browser
    ) {
        // Init dependencies
        $this->browser = $browser;

        // Init header
        $this->gtk = new \GtkHeaderBar;

        $this->gtk->set_show_close_button(
            $this->_actions
        );

        $this->gtk->set_title(
            $this->_title
        );

        $this->gtk->set_subtitle(
            $this->_subtitle
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
        ?string $value = null,
        ?string $subtitle = null
    ): void
    {
        $this->gtk->set_title(
            is_null($value) ? $this->_title : sprintf(
                '%s - %s',
                trim(
                    $value
                ),
                $this->_title
            )
        );

        $this->setSubtitle(
            $subtitle
        );
    }

    public function setSubtitle(
        ?string $value = null
    ): void
    {
        $this->gtk->set_subtitle(
            is_null($value) ? $this->_subtitle : trim(
                $value
            )
        );
    }
}