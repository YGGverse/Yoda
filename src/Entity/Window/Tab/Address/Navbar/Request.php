<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Navbar;

class Request extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\Address\Navbar\Entry
{
    private string $_placeholder = 'URL or search term...';

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        $this->navbar->address->update();
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->navbar->base->refresh();

        $this->navbar->go->gtk->set_sensitive(
            !empty(
                $entry->get_text()
            )
        );
    }
}