<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar;

abstract class Entry extends \Yggverse\Yoda\Abstract\Entity\Entry
{
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar $navbar;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar $navbar
    ) {
        parent::__construct();

        $this->navbar = $navbar;
    }
}
