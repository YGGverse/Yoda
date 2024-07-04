<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab;

use \Yggverse\Yoda\Entity\Window\Tab\History\Title;
use \Yggverse\Yoda\Entity\Window\Tab\History\Navbar;
use \Yggverse\Yoda\Entity\Window\Tab\History\Content;

class History
{
    public \GtkBox $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab $tab;

    public \Yggverse\Yoda\Entity\Window\Tab\History\Title $title;
    public \Yggverse\Yoda\Entity\Window\Tab\History\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Window\Tab\History\Content $content;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab $tab
    ) {
        $this->tab = $tab;

        $this->title = new Title(
            $this
        );

        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->content = new Content(
            $this
        );

        $this->navbar = new Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        $this->gtk->pack_start(
            $this->content->gtk,
            true,
            true,
            0
        );
    }

    public function search(
        ?string $filter = null
    ): void
    {
        $this->navbar->filter->setValue(
            trim(
                strval(
                    $filter
                )
            )
        );

        $this->content->search(
            trim(
                strval(
                    $filter
                )
            )
        );
    }
}