<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Menu\Item;

class Yoda
{
    public \GtkMenuItem $item;

    public function __construct(string $label = 'Yoda')
    {
        $this->item = \GtkMenuItem::new_with_label(
            $label
        );

        $children = new \GtkMenu();

        $quit = new \Yggverse\Yoda\Menu\Item\Quit();

        $children->append(
            $quit->item
        );

        $this->item->set_submenu(
            $children
        );
    }
}