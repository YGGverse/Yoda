<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Navbar\History;

class Forward extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Tab\Page\Navbar\Button
{
    protected string $_label = 'Forward';

    protected function _onCLick(
        \GtkButton $entity
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

        $this->navbar->history->refresh();
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