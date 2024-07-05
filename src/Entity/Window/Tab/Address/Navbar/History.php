<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Navbar;

use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History\Back;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History\Forward;

class History
{
    public \GtkButtonBox $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History\Back $back;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar\History\Forward $forward;

    public \Yggverse\Yoda\Model\History $memory;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar $navbar
    ) {
        $this->memory = new \Yggverse\Yoda\Model\History();

        $this->navbar = $navbar;

        $this->gtk = new \GtkButtonBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_layout(
            \GtkButtonBoxStyle::EXPAND
        );

        $this->back = new Back(
            $this->navbar
        );

        $this->gtk->add(
            $this->back->gtk
        );

        $this->forward = new Forward(
            $this->navbar
        );

        $this->gtk->add(
            $this->forward->gtk
        );
    }

    public function add(
        string $value
    ): void
    {
        if (empty($value))
        {
            throw new \Exception;
        }

        if ($value != $this->memory->getCurrent())
        {
            $this->memory->add(
                $value
            );
        }

        $this->refresh();
    }

    public function refresh(): void
    {
        $this->back->gtk->set_sensitive(
            (bool) $this->memory->getBack()
        );

        $this->forward->gtk->set_sensitive(
            (bool) $this->memory->getForward()
        );
    }
}