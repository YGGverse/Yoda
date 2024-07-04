<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window;

class Header
{
    public \GtkHeaderBar $gtk;

    public \Yggverse\Yoda\Entity\Window $window;

    // Defaults
    private bool $_actions = true;
    private string $_title = 'Yoda';

    public function __construct(
        \Yggverse\Yoda\Entity\Window $window
    ) {
        $this->window = $window;

        $this->gtk = new \GtkHeaderBar;

        $this->gtk->set_show_close_button(
            $this->_actions
        );

        $this->setTitle(
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