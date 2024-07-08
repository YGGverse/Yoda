<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Navbar;

class Go extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Tab\Page\Navbar\Button
{
    protected string $_label = 'Go';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->navbar->page->update();
    }

    public function refresh(): void
    {
        $this->gtk->set_sensitive(
            !empty(
                $this->navbar->request->getValue()
            )
        );
    }
}