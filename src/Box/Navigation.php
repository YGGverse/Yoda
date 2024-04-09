<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Box;

class Navigation
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Entry\Address $address;

    public \Yggverse\Yoda\Button\Home $home;
    public \Yggverse\Yoda\Button\Back $back;
    public \Yggverse\Yoda\Button\Forward $forward;
    public \Yggverse\Yoda\Button\Reload $reload;
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

        if ($config->interface->window->navigation->button->home)
        {
            $this->home = new \Yggverse\Yoda\Button\Home();

            $this->box->pack_start(
                $this->home->button,
                false,
                false,
                8
            );
        }

        if ($config->interface->window->navigation->button->back)
        {
            $this->back = new \Yggverse\Yoda\Button\Back();

            $this->box->pack_start(
                $this->back->button,
                false,
                false,
                8
            );
        }

        if ($config->interface->window->navigation->button->forward)
        {
            $this->forward = new \Yggverse\Yoda\Button\Forward();

            $this->box->pack_start(
                $this->forward->button,
                false,
                false,
                8
            );
        }

        if ($config->interface->window->navigation->button->reload)
        {
            $this->reload = new \Yggverse\Yoda\Button\Reload();

            $this->box->pack_start(
                $this->reload->button,
                false,
                false,
                8
            );
        }

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