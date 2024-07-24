<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \Exception;
use \GtkButtonBox;
use \GtkButtonBoxStyle;
use \GtkOrientation;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \Yggverse\Yoda\Model\History as Memory;

class History
{
    // GTK
    public GtkButtonBox $gtk;

    // Dependencies
    public Navbar $navbar;
    public Memory $memory;

    // Requirements
    public History\Back $back;
    public History\Forward $forward;

    public function __construct(
        Navbar $navbar
    ) {
        $this->memory = new Memory();

        $this->navbar = $navbar;

        $this->gtk = new GtkButtonBox(
            GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_layout(
            GtkButtonBoxStyle::EXPAND
        );

        $this->back = new History\Back(
            $this->navbar
        );

        $this->gtk->add(
            $this->back->gtk
        );

        $this->forward = new History\Forward(
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
            throw new Exception;
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