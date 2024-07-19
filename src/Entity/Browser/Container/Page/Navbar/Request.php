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

    // Update setter with session update feature
    public function setValue(
        ?string $value = null
    ): void
    {
        $this->gtk->set_text(
            is_null($value) ? $this->_value : trim(
                strval(
                    $value
                )
            )
        );

        // Update session on tab initiated only
        if (isset($this->navbar->page->container->tab))
        {
            $this->navbar->page->container->tab->updateSession();
        }
    }
}