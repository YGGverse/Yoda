<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History\Navbar;

class Search extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\History\Navbar\Button
{
    protected bool   $_sensitive = true;
    protected string $_label = 'Search';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->gtk->set_sensitive(
            false
        );

        $this->navbar->history->content->search(
            $this->navbar->filter->gtk->get_text()
        );

        $this->gtk->set_sensitive(
            true
        );
    }
}
