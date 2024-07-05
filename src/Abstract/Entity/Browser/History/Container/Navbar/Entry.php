<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar;

abstract class Entry extends \Yggverse\Yoda\Abstract\Entity\Entry
{
    public \Yggverse\Yoda\Entity\Browser\History\Container\Navbar $navbar;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\History\Container\Navbar $navbar
    ) {
        parent::__construct();

        $this->navbar = $navbar;
    }
}
