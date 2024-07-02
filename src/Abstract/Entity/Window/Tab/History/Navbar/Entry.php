<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Window\Tab\History\Navbar;

abstract class Entry extends \Yggverse\Yoda\Abstract\Entity\Entry
{
    public \Yggverse\Yoda\Entity\Window\Tab\History\Navbar $navbar;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\History\Navbar $navbar
    ) {
        parent::__construct();

        $this->navbar = $navbar;
    }
}
