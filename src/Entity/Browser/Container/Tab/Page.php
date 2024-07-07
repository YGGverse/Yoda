<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab;

use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Title;
use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Navbar;
use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content;

class Page
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Tab $tab;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Title $title;
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content $content;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Tab $tab
    ) {
        // Init dependencies
        $this->tab = $tab;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        // Init title
        $this->title = new Title(
            $this
        );

        // Init navbar
        $this->navbar = new Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        // Init content
        $this->content = new Content(
            $this
        );

        $this->gtk->add(
            $this->content->gtk
        );

        // Render
        $this->gtk->show_all();
    }

    public function refresh(): void
    {
        $this->navbar->refresh();
        $this->content->refresh();
    }

    public function update(
        bool $history = true
    ): void
    {
        // @TODO navbar

        $this->content->update(
            $history
        );
    }
}