<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

class Tab
{
    public \GtkNotebook $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container $container;

    // Defaults
    private bool $_reorderable = true;
    private bool $_scrollable  = true;

    // Extras
    private array $_page = [];

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
        $this->appendPage(
            'gemini://yggverse.cities.yesterweb.org'
        );

        $this->appendPage(
            'gemini://tlgs.one'
        );

        $this->appendPage(
            'nex://nightfall.city'
        );

        // Init events
        $this->gtk->connect(
            'switch-page',
            function (
                ?\GtkNotebook $self,
                ?\GtkWidget $child,
                int $page_num
            ) {
                // Update header bar title
                $this->container->browser->header->setTitle(
                    $this->getPage($page_num)->title->getValue(),
                    $this->getPage($page_num)->title->getSubtitle()
                );

                // Keep current selection
                $self->grab_focus();
            }
        );

        $this->gtk->connect(
            'page-removed',
            function (
                ?\GtkNotebook $self,
                ?\GtkWidget $child,
                int $page_num
            ) {
                $this->closePage(
                    $page_num
                );
            }
        );

        $this->gtk->connect(
            'page-reordered',
            function (
                ?\GtkNotebook $self,
                ?\GtkWidget $child,
                int $page_num
            ) {
                $this->reorderPage(
                    $page_num
                );
            }
        );
    }

    public function appendPage(
        ?string $request = null,
        bool $focus = true
    ): void
    {
        // Extendable classes not supported by PHP-GTK3 #117
        // create internal pages registry
        $this->_page[] = $page = new Page(
            $this->container
        );

        $this->gtk->append_page(
            $page->gtk,
            $page->title->gtk
        );

        $this->gtk->set_tab_reorderable(
            $page->gtk,
            $this->_reorderable
        );

        if ($request)
        {
            $page->open(
                $request
            );
        }

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
                $page->title->getValue(),
                $page->title->getSubtitle()
            );
        }

        // Render
        $this->gtk->show();
    }

    public function getPage(
        int $page_num
    ): ?\Yggverse\Yoda\Entity\Browser\Container\Page
    {
        if (empty($this->_page[$page_num]))
        {
            throw new \Exception;
        }

        return $this->_page[$page_num];
    }

    public function closePage(
        int $page_num
    ): void
    {
        if (empty($this->_page[$page_num]))
        {
            throw new \Exception;
        }

        // Remove GTK node
        $this->gtk->remove_page(
            $page_num
        );

        // Free memory
        $this->_page[$page_num] = null;

        // Cleanup internal record
        unset(
            $this->_page[$page_num]
        );

        // Reorder @TODO
    }

    public function reorderPage(
        int $page_num
    ): void
    {
        /* @TODO
        $this->_page = array_splice(
            $this->_page,
            $page_num,
            0,
            // @TODO get $page
        ); */
    }
}