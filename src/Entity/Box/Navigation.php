<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Box;

class Navigation
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Entity\Entry\Address $address;

    public \Yggverse\Yoda\Entity\Button\Home $home;
    public \Yggverse\Yoda\Entity\Button\Back $back;
    public \Yggverse\Yoda\Entity\Button\Forward $forward;
    public \Yggverse\Yoda\Entity\Button\Reload $reload;
    public \Yggverse\Yoda\Entity\Button\Go $go;

    public object $config;

    public function __construct(
        string $name = 'boxNavigation'
    ) {
        $this->config = \Yggverse\Yoda\Model\File::getConfig();

        $this->box = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->box->set_name(
            $name
        );

        if ($this->config->interface->window->navigation->button->home && $this->config->homepage)
        {
            $this->home = new \Yggverse\Yoda\Entity\Button\Home();

            $this->box->pack_start(
                $this->home->button,
                false,
                false,
                8
            );
        }

        if ($this->config->interface->window->navigation->button->back || $this->config->interface->window->navigation->button->forward)
        {
            $boxBackForward = new \GtkButtonBox(
                \GtkOrientation::HORIZONTAL
            );

            $boxBackForward->set_layout(
                \GtkButtonBoxStyle::EXPAND
            );

            if ($this->config->interface->window->navigation->button->back)
            {
                $this->back = new \Yggverse\Yoda\Entity\Button\Back();

                $boxBackForward->pack_start(
                    $this->back->button,
                    false,
                    true,
                    0
                );
            }

            if ($this->config->interface->window->navigation->button->forward)
            {
                $this->forward = new \Yggverse\Yoda\Entity\Button\Forward();

                $boxBackForward->pack_end(
                    $this->forward->button,
                    false,
                    true,
                    0
                );
            }

            $this->box->pack_start(
                $boxBackForward,
                false,
                false,
                8
            );
        }

        if ($this->config->interface->window->navigation->button->reload)
        {
            $this->reload = new \Yggverse\Yoda\Entity\Button\Reload();

            $this->box->pack_start(
                $this->reload->button,
                false,
                false,
                8
            );
        }

        $this->address = new \Yggverse\Yoda\Entity\Entry\Address(
            $this->config->homepage
        );

        $this->box->pack_start(
            $this->address->entry,
            true,
            true,
            8
        );

        if ($this->config->interface->window->navigation->button->go)
        {
            $this->go = new \Yggverse\Yoda\Entity\Button\Go();

            $this->box->pack_end(
                $this->go->button,
                false,
                false,
                8
            );
        }
    }
}