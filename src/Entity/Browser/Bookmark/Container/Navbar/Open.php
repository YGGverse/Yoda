<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Bookmark\Container\Navbar;

use \GtkButton;

use \Yggverse\Yoda\Abstract\Entity\Browser\Bookmark\Container\Navbar\Button;

class Open extends Button
{
    public const IMAGE = null; // list-add-symbolic | tab-new-symbolic
    public const LABEL = 'Open';
    public const TOOLTIP = 'Open';

    protected function _onCLick(
        GtkButton $entity
    ): void
    {
        $this->navbar->container->bookmark->browser->container->tab->append(
            $this->navbar->container->content->table->getSelectedRequest()
        );
    }

    public function refresh(): void
    {
        $this->gtk->set_sensitive(
            boolval(
                $this->navbar->container->content->table->getSelectedId()
            )
        );
    }
}
