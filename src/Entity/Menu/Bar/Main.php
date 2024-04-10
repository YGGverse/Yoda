<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Menu\Bar;

class Main
{
    public \GtkMenuBar $bar;

    public \Yggverse\Yoda\Entity\Menu\Item\Yoda $yoda;

    public function __construct()
    {
        $this->bar = new \GtkMenuBar();

        $this->yoda = new \Yggverse\Yoda\Entity\Menu\Item\Yoda();

        $this->bar->append(
            $this->yoda->item
        );
    }
}