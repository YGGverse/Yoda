<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Box;

class Navigation
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Entry\Address $address;
    public \Yggverse\Yoda\Button\Go $go;

    public function __construct(
        string $name = 'boxNavigation'
    ) {
        global $config;

        $this->box = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->box->set_name(
            $name
        );

        $this->address = new \Yggverse\Yoda\Entry\Address(
            $config->homepage
        );

        $this->box->pack_start(
            $this->address->entry,
            true,
            true,
            8
        );

        if ($config->interface->window->navigation->button->go)
        {
            $this->go = new \Yggverse\Yoda\Button\Go();

            $this->box->pack_end(
                $this->go->button,
                false,
                false,
                8
            );
        }
    }
}