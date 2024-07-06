<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \Yggverse\Yoda\Entity\Browser\Header\Navigation;

class Header
{
    public \GtkHeaderBar $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser $browser;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Header\Navigation $navigation;

    // Defaults
    protected bool $_actions = true;
    protected string $_title = 'Yoda';
    protected string $_subtitle = '';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser $browser
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

        // Init navigation
        $this->navigation = new Navigation(
            $this
        );

        $this->gtk->add(
            $this->navigation->gtk
        );
    }

    public function setTitle(
        ?string $title = null
    ): void
    {
        $this->gtk->set_title(
            is_null($title) ? $this->_title : sprintf(
                '%s - %s',
                trim(
                    $title
                ),
                $this->_title
            )
        );
    }
}