<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History\Navbar;

use \Yggverse\Yoda\Entity\Window\Tab\Address;

class Open extends \Yggverse\Yoda\Abstract\Entity\Window\Tab\History\Navbar\Button
{
    protected string $_label = 'Open';

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $address = new Address(
            $this->navbar->history->tab
        );

        $this->navbar->history->tab->append( // @TODO
            $address
        );

        $address->navbar->request->setValue(
            $this->navbar->history->content->getSelectedUrl()
        );

        $address->update();
    }
}
