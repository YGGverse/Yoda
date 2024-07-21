<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

use \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Button;

class Delete extends Button
{
    public const LABEL = 'Delete';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        if ($id = $this->navbar->container->content->table->getSelectedId())
        {
            $this->navbar->container->history->browser->database->deleteHistory(
                $id
            );
        }

        $this->navbar->container->refresh();
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
