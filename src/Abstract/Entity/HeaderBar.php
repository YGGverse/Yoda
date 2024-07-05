<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class HeaderBar
{
    public \GtkHeaderBar $gtk;

    protected bool $_actions = true;
    protected string $_title = 'Yoda';

    public function __construct()
    {
        $this->gtk = new \GtkHeaderBar;

        $this->gtk->set_show_close_button(
            $this->_actions
        );

        $this->gtk->set_title(
            $this->_title
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