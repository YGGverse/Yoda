<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

class Request extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Entry
{
    protected string $_placeholder = 'URL or search term...';

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        $this->navbar->page->content->update();
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->navbar->refresh();
    }
}