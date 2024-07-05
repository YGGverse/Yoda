<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History;

class Back extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\Address\Navbar\Button
{
    protected string $_label = 'Back';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        if ($this->navbar->history->memory->getBack())
        {
            $this->navbar->request->setValue(
                $this->navbar->history->memory->goBack()
            );

            $this->navbar->address->update(
                false
            );
        }

        $this->navbar->history->refresh();
    }
}