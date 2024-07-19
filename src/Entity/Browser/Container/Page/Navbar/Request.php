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
        $this->navbar->page->open(
            $entry->get_text()
        );

        $this->navbar->page->container->tab->updateSession();
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->navbar->refresh();

        $this->navbar->page->container->tab->updateSession();
    }

    protected function _onChanged(
        \GtkEntry $entry
    ): void
    {
        // Update session on tab initiated only
        if (isset($this->navbar->page->container->tab))
        {
            \Gtk::timeout_add(
                1000, // wait for one second to apply changes
                function()
                {
                    $this->navbar->page->container->tab->updateSession();

                    return false; // stop
                }
            );
        }
    }
}