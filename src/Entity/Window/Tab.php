<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window;

use \Yggverse\Yoda\Entity\Window\Tab\Address;
use \Yggverse\Yoda\Entity\Window\Tab\History;

class Tab
{
    public \GtkNotebook $gtk;

    public \Yggverse\Yoda\Entity\Window $window;

    // Defaults
    private bool $_reorderable = true;
    private bool $_scrollable  = true;

    public function __construct(
        \Yggverse\Yoda\Entity\Window $window
    ) {
        $this->window = $window;

        $this->gtk = new \GtkNotebook;

        $this->gtk->set_scrollable(
            $this->_scrollable
        );

        $this->gtk->connect(
            'switch-page',
            function (
                \GtkNotebook $entity,
                \GtkWidget $child,
                int $position
            ) {
                $this->window->header->setTitle(
                    $entity->get_tab_label(
                        $child
                    )->get_text()
                );
            }
        );

        $this->append( // @TODO remove
            new History(
                $this
            )
        );

        $this->append( // @TODO remove
            new Address(
                $this
            )
        );
    }

    public function append(
        Address | History $entity,
        ?bool $reorderable = null
    ): void
    {
        $this->gtk->append_page(
            $entity->gtk,
            $entity->title->gtk
        );

        $this->gtk->set_tab_reorderable(
            $entity->gtk,
            is_null($reorderable) ? $this->_reorderable : $reorderable
        );

        $this->gtk->show_all();

        // Focus on appended tab
        $this->gtk->set_current_page(
            $this->gtk->page_num(
                $entity->gtk
            )
        );

        // Update application title
        $this->window->header->setTitle(
            $entity->title->gtk->get_text()
        );
    }
}