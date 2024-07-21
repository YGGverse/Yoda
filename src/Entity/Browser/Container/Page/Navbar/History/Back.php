<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\History;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Button;

class Back extends Button
{
    public const LABEL = 'Back';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        if ($this->navbar->history->memory->getBack())
        {
            $this->navbar->request->setValue(
                $this->navbar->history->memory->goBack()
            );

            $this->navbar->page->update(
                false
            );
        }

        $this->navbar->refresh();
    }

    public function refresh(): void
    {
        $this->gtk->set_sensitive(
            boolval(
                $this->navbar->history->memory->getBack()
            )
        );
    }
}