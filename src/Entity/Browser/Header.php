<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser;

use \Yggverse\Yoda\Entity\Browser\Header\Navigation;
use \Yggverse\Yoda\Entity\Browser\Header\Tab;

class Header
{
    public \GtkHeaderBar $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser $browser;

    // Requirements
    public Navigation $navigation;
    public Tab $tab;

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

        // Init new tab button
        $this->tab = new Tab(
            $this
        );

        $this->gtk->add(
            $this->tab->gtk
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