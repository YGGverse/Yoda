<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History;

class Header
{
    public \GtkHeaderBar $gtk;

    public const ACTIONS = true;
    public const TITLE = 'History - Yoda';
    public const SUBTITLE = '';

    public function __construct()
    {
        $this->gtk = new \GtkHeaderBar;

        $this->gtk->set_show_close_button(
            $this::ACTIONS
        );

        $this->gtk->set_title(
            _($this::TITLE)
        );

        $this->gtk->set_subtitle(
            _($this::SUBTITLE)
        );

        $this->gtk->show();
    }
}