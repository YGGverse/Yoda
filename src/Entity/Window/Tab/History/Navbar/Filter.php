<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History\Navbar;

class Filter extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\History\Navbar\Entry
{
    private string $_placeholder = 'Search in history...';

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        $this->navbar->history->content->search(
            $this->navbar->filter->gtk->get_text()
        );
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->navbar->history->content->search(
            $this->navbar->filter->gtk->get_text()
        );
    }
}