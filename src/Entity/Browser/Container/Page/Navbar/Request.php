<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \GdkEvent;
use \Gtk;
use \GtkEntry;

use \Yggverse\Yoda\Abstract\Entity\Entry;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

class Request extends Entry
{
    // Defaults
    public const PLACEHOLDER = 'URL or search term...';

    // Extras
    private ?int $_changed = null;

    // Dependencies
    public Navbar $navbar;

    // Requirements
    public Request\Completion $completion;

    public function __construct(
        Navbar $navbar
    ) {
        // Build entry
        parent::__construct();

        // Dependencies
        $this->navbar = $navbar;

        // Requirements
        $this->completion = new Request\Completion(
            $this
        );

        $this->gtk->set_completion(
            $this->completion->gtk
        );
    }

    protected function _onActivate(
        GtkEntry $entry
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
        GtkEntry $entry,
        GdkEvent $event
    ): void
    {
        // Delegate
        $this->_onChanged(
            $entry
        );
    }

    protected function _onChanged(
        GtkEntry $entry
    ): void
    {
        // Refresh navigation elements
        $this->navbar->refresh();

        // Show suggestions autocomplete
        $this->completion->refresh();

        // Update session
        if (isset($this->navbar->page->container->tab))
        {
            // Reset keyup time
            if ($this->_changed)
            {
                Gtk::source_remove(
                    $this->_changed
                );

                $this->_changed = null;
            }

            // Wait for one second to apply act
            $this->_changed = Gtk::timeout_add(
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
        GtkEntry $entry,
        GdkEvent $event
    ): void
    {}
}