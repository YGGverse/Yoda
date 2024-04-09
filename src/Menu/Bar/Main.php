<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Menu\Bar;

class Main
{
    public \GtkMenuBar $bar;

    public \Yggverse\Yoda\Menu\Item\Yoda $yoda;

    public function __construct()
    {
        $this->bar = new \GtkMenuBar();

        $this->yoda = new \Yggverse\Yoda\Menu\Item\Yoda();

        $this->bar->append(
            $this->yoda->item
        );
    }
}