<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Menu\Item;

class Quit
{
    public \GtkMenuItem $item;

    public function __construct(string $label = 'Quit')
    {
        $this->item = \GtkMenuItem::new_with_label(
            $label
        );

        $this->activate();
    }

    public function activate(): void
    {
        $this->item->connect(
            'activate',
            function ()
            {
                \Gtk::main_quit();
            }
        );
    }
}