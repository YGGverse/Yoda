<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Box;

class Tab
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Entity\Box\Menu $menu;
    public \Yggverse\Yoda\Entity\Box\Navigation $navigation;
    public \Yggverse\Yoda\Entity\Label\Content $content;
    public \Yggverse\Yoda\Entity\Label\Tray $tray;

    public function __construct(
        string $name = 'boxTab'
    ) {
        // Init container
        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->box->set_name(
            $name
        );

        // Init dependencies
        $this->menu = new \Yggverse\Yoda\Entity\Box\Menu();

        $this->box->pack_start(
            $this->menu->box,
            false,
            true,
            0
        );

        $this->navigation = new \Yggverse\Yoda\Entity\Box\Navigation();

        $this->box->pack_start(
            $this->navigation->box,
            false,
            true,
            8
        );

        $this->content = new \Yggverse\Yoda\Entity\Label\Content();

        $scroll = new \GtkScrolledWindow();

        $scroll->add(
            $this->content->label
        );

        $this->box->pack_start(
            $scroll,
            true,
            true,
            0
        );

        $this->tray = new \Yggverse\Yoda\Entity\Label\Tray();

        $this->box->pack_start(
            $this->tray->label,
            false,
            true,
            0
        );
    }
}