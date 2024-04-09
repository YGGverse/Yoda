<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Box;

class Navigation
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Entry\Address $address;

    public function __construct(
        string $name = 'boxNavigation'
    ) {
        global $config;

        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->box->set_name(
            $name
        );

        $this->address = new \Yggverse\Yoda\Entry\Address(
            $config->homepage
        );

        $this->box->add(
            $this->address->entry
        );
    }
}