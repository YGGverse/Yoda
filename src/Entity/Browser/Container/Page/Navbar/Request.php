<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

class Request extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Entry
{
    protected string $_placeholder = 'URL or search term...';

    private ?int $_changed = null;

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        $this->navbar->page->open(
            $entry->get_text()
        );

        $this->navbar->page->container->tab->update();
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->navbar->refresh();

        $this->navbar->page->container->tab->update();
    }

    protected function _onChanged(
        \GtkEntry $entry
    ): void
    {
        // Update session on tab initiated only
        if (isset($this->navbar->page->container->tab))
        {
            // Reset previous event
            if ($this->_changed)
            {
                // @TODO source_remove #125
            }

            // Wait for one second to apply act
            $this->_changed = \Gtk::timeout_add(
                1000,
                function()
                {
                    $this->navbar->page->container->tab->update();

                    return false; // stop
                }
            );
        }
    }

    protected function _onFocusOut(
        \GtkEntry $entry
    ): void
    {}
}