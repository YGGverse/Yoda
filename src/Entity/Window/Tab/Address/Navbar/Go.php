<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Navbar;

class Go extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\Address\Navbar\Button
{
    protected string $_label = 'Go';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->navbar->address->update();
    }
}