<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Box;

class Menu
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Menu\Bar\Main $main;

    public function __construct(
        string $name = 'boxMenu'
    ) {
        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->box->set_name(
            $name
        );

        $this->main = new \Yggverse\Yoda\Menu\Bar\Main();

        $this->box->add(
            $this->main->bar
        );
    }
}