<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \GtkButton;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Button;

class Update extends Button
{
    public const IMAGE = 'view-refresh-symbolic';
    public const LABEL = 'Go';
    public const TOOLTIP = 'Update';

    protected function _onCLick(
        GtkButton $entity
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