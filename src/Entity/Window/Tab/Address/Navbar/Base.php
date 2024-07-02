<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Navbar;

class Base extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\Address\Navbar\Button
{
    protected string $_label = 'Base';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $address = new \Yggverse\Net\Address(
            $this->navbar->request->gtk->get_text()
        );

        if ($address->getHost())
        {
            $this->navbar->request->gtk->set_text(
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

        $this->update();
    }

    public function update(
        ?\Yggverse\Net\Address $address = null
    ): void
    {
        if (is_null($address))
        {
            $address = new \Yggverse\Net\Address(
                $this->navbar->request->gtk->get_text()
            );
        }

        $this->navbar->base->gtk->set_sensitive(
            $address->getHost() && ($address->getPath() || $address->getQuery())
        );
    }
}