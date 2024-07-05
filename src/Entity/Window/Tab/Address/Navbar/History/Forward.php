<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History;

class Forward extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\Address\Navbar\Button
{
    protected string $_label = 'Forward';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        if ($this->navbar->history->memory->getForward())
        {
            $this->navbar->request->setValue(
                $this->navbar->history->memory->goForward()
            );

            $this->navbar->address->update(
                false
            );
        }

        $this->navbar->history->refresh();
    }
}