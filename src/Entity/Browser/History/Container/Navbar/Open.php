<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

class Open extends \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Button
{
    protected string $_label = 'Open';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->navbar->container->history->browser->container->tab->appendPage(
            $this->navbar->container->content->table->getSelectedUrl()
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
