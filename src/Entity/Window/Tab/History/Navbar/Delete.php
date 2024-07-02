<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History\Navbar;

class Delete extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\History\Navbar\Button
{
    protected string $_label = 'Delete';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        if ($id = $this->navbar->history->content->getSelectedId())
        {
            $this->navbar->history->tab->window->database->deleteHistory(
                $id
            );

            $this->navbar->open->gtk->set_sensitive(
                false
            );

            $this->navbar->history->content->search(
                $this->navbar->filter->gtk->get_text()
            );
        }
    }
}
