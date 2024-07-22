<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Entry;

class Request extends Entry
{
    public const PLACEHOLDER = 'URL or search term...';

    private ?int $_changed = null;

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        if (empty($this->getValue()))
        {
            return;
        }

        $this->navbar->page->open(
            $this->getValue()
        );

        $this->navbar->page->container->tab->update();
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        // Delegate
        $this->_onChanged(
            $entry
        );
    }

    protected function _onChanged(
        \GtkEntry $entry
    ): void
    {
        // Refresh navigation elements
        $this->navbar->refresh();

        // Update session on tab initiated only
        if (isset($this->navbar->page->container->tab))
        {
            // Reset previous event
            if ($this->_changed)
            {
                \Gtk::source_remove(
                    $this->_changed
                );

                $this->_changed = null;
            }

            // Wait for one second to apply act
            $this->_changed = \Gtk::timeout_add(
                1000,
                function()
                {
                    $this->navbar->page->container->tab->update();

                    $this->_changed = null;

                    return false; // stop
                }
            );
        }
    }

    protected function _onFocusOut(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {}
}