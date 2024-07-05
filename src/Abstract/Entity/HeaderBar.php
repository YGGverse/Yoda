<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class HeaderBar
{
    public \GtkHeaderBar $gtk;

    protected bool $_actions = true;
    protected string $_title = 'Yoda';
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