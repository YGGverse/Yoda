<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

class Tab
{
    public \GtkNotebook $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container $container;

    // Defaults
    private bool $_reorderable = true;
    private bool $_scrollable  = true;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container $container
    ) {
        // Init dependency
        $this->container = $container;

        // Init container
        $this->gtk = new \GtkNotebook;

        $this->gtk->set_scrollable(
            $this->_scrollable
        );

        // Init previous session @TODO
        $this->append(
            'gemini://yggverse.cities.yesterweb.org'
        );

        // Connect events
        $this->gtk->connect(
            'switch-page',
            function (
                \GtkNotebook $entity,
                \GtkWidget $child,
                int $position
            ) {
                $this->container->browser->header->setTitle(
                    $entity->get_tab_label(
                        $child
                    )->get_text()
                );

                // Keep current selection
                $entity->grab_focus();
            }
        );
    }

    public function append(
        ?string $request = null,
        bool $focus = true
    ): void
    {
        $page = new \Yggverse\Yoda\Entity\Browser\Container\Tab\Page(
            $this
        );

        if ($request)
        {
            $page->navbar->request->setValue(
                $request
            );

            $page->content->update();
        }

        $this->gtk->append_page(
            $page->gtk,
            $page->title->gtk
        );

        $this->gtk->set_tab_reorderable(
            $page->gtk,
            $this->_reorderable
        );

        if ($focus)
        {
            // Focus on appended tab
            $this->gtk->set_current_page(
                $this->gtk->page_num(
                    $page->gtk
                )
            );

            // Update application title
            $this->container->browser->header->setTitle(
                $page->title->gtk->get_text()
            );
        }

        $this->gtk->show_all();
    }
}