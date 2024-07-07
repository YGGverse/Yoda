<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

class Filter extends \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Entry
{
    private string $_placeholder = 'Search in history...';

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        $this->navbar->container->content->search(
            $entry->get_text()
        );
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->navbar->container->content->search(
            $entry->get_text()
        );
    }
}