<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History;

class Header
{
    public \GtkHeaderBar $gtk;

    protected bool $_actions = true;
    protected string $_title = 'History - Yoda';
    protected string $_subtitle = '';

    public function __construct()
    {
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

        // Render
        $this->gtk->show();
    }
}