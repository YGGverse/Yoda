<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header;

use \Yggverse\Yoda\Entity\Browser\Header;

use \Yggverse\Yoda\Entity\Browser\Header\Tray\Navigation;
use \Yggverse\Yoda\Entity\Browser\Header\Tray\Tab;

class Tray
{
    public \GtkBox $gtk;

    // Dependencies
    public Header $header;

    // Requirements
    public Navigation $navigation;
    public Tab $tab;

    // Defaults
    protected bool $_actions = true;
    protected string $_title = 'Yoda';
    protected string $_subtitle = '';
    protected int $_margin = 8;

    public function __construct(
        Header $header
    ) {
        // Init dependencies
        $this->header = $header;

        // Init header
        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_margin_start(
            $this->_margin / 2
        );

        $this->gtk->set_margin_end(
            $this->_margin / 2
        );

        $this->gtk->set_spacing(
            $this->_margin
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
}