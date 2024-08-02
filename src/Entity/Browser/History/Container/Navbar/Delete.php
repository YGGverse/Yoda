<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Navbar;

use \GtkButton;

use \Yggverse\Yoda\Abstract\Entity\Browser\History\Container\Navbar\Button;

class Delete extends Button
{
    public const IMAGE = 'edit-delete-symbolic';
    public const LABEL = 'Delete';
    public const TOOLTIP = 'Delete';

    protected function _onCLick(
        GtkButton $entity
    ): void
    {
        if ($id = $this->navbar->container->content->table->getSelectedId())
        {
            $this->navbar->container->history->browser->database->history->delete(
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
