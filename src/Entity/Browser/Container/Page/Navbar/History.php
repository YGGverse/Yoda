<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\History\Back;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\History\Forward;

use \Yggverse\Yoda\Model\History as Memory;

class History
{
    public \GtkButtonBox $gtk;

    // Dependencies
    public Navbar $navbar;
    public Memory $memory;

    // Requirements
    public Back $back;
    public Forward $forward;

    public function __construct(
        Navbar $navbar
    ) {
        $this->memory = new Memory();

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

        // Render
        $this->gtk->show();
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
        $this->back->refresh();
        $this->forward->refresh();
    }
}