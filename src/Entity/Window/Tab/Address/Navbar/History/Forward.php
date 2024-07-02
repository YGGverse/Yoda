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
        // @TODO
    }
}