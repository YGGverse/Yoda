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
        // @TODO
    }
}