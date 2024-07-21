<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

use \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Entry;

class Filter extends Entry
{
    public const PLACEHOLDER = 'Search in history...';

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

    protected function _onChanged(
        \GtkEntry $entry
    ): void
    {}

    protected function _onFocusOut(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {}
}