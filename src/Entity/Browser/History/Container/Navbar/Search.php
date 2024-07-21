<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

class Search extends \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Button
{
    public const SENSITIVE = true;
    public const LABEL = 'Search';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->navbar->container->content->search(
            $this->navbar->filter->getValue()
        );
    }
}
