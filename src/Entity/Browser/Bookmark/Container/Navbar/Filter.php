<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Bookmark\Container\Navbar;

use \GdkEvent;
use \GtkEntry;

use \Yggverse\Yoda\Abstract\Entity\Browser\Bookmark\Container\Navbar\Entry;

class Filter extends Entry
{
    public const PLACEHOLDER = 'Search in bookmarks...';

    protected function _onActivate(
        GtkEntry $entry
    ): void
    {
        $this->navbar->container->content->search(
            $entry->get_text()
        );
    }

    protected function _onKeyRelease(
        GtkEntry $entry,
        GdkEvent $event
    ): void
    {
        $this->navbar->container->content->search(
            $entry->get_text()
        );
    }

    protected function _onChanged(
        GtkEntry $entry
    ): void
    {}

    protected function _onFocusOut(
        GtkEntry $entry,
        GdkEvent $event
    ): void
    {}
}