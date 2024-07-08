<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

class Search extends \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Button
{
    protected bool   $_sensitive = true;
    protected string $_label = 'Search';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->navbar->container->content->search(
            $this->navbar->filter->getValue()
        );
    }
}
