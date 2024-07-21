<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar;

use \Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

abstract class Button extends \Yggverse\Yoda\Abstract\Entity\Button
{
    public Navbar $navbar;

    public function __construct(
        Navbar $navbar
    ) {
        parent::__construct();

        $this->navbar = $navbar;
    }
}
