<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Navbar;

class Base extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Tab\Page\Navbar\Button
{
    protected string $_label = 'Base';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $address = new \Yggverse\Net\Address(
            trim(
                strval(
                    $this->navbar->request->gtk->get_text()
                )
            )
        );

        if ($address->getHost())
        {
            $this->navbar->request->setValue(
                $address->get( // build base
                    true,
                    true,
                    true,
                    true,
                    true,
                    false,
                    false,
                    false
                )
            );

            $this->navbar->address->update();
        }

        $this->refresh();
    }

    public function refresh(): void
    {
        $address = new \Yggverse\Net\Address(
            rtrim(
                $this->navbar->request->gtk->get_text(),
                '/'
            )
        );

        $this->gtk->set_sensitive(
            $address->getHost() && (
                $address->getPath() || $address->getQuery()
            )
        );
    }
}