<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\History;

use \GtkButton;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Button;

class Forward extends Button
{
    public const IMAGE = 'go-next-symbolic';
    public const LABEL = 'Forward';
    public const TOOLTIP = 'Forward';

    protected function _onCLick(
        GtkButton $entity
    ): void
    {
        if ($this->navbar->history->memory->getForward())
        {
            $this->navbar->request->setValue(
                $this->navbar->history->memory->goForward()
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
                $this->navbar->history->memory->getForward()
            )
        );
    }
}