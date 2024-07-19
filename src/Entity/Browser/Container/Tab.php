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

        // Restore previous session
        foreach ($this->container->browser->database->getSession() as $session)
        {
            $this->appendPage(
                $session->request
            );
        }

        // Init events
        $this->gtk->connect(
            'switch-page',
            function (
                ?\GtkNotebook $self,
                ?\GtkWidget $child,
                int $page_num
            ) {
                // Update header bar title
                if ($page = $this->getPage($page_num))
                {
                    $this->container->browser->header->setTitle(
                        $page->title->getValue(),
                        $page->title->getSubtitle()
                    );
                }

                // Keep current selection
                $self->grab_focus();
            }
        );

        $this->gtk->connect(
            'page-added',
            function (
                ?\GtkNotebook $self,
                ?\GtkWidget $child,
                int $page_num
            ) {
                $this->reorderPage(
                    null // all
                );
            }
        );

        $this->gtk->connect(
            'page-removed',
            function (
                ?\GtkNotebook $self,
                ?\GtkWidget $child,
                int $page_num
            ) {
                $this->reorderPage(
                    null // all
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
                    null // all
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
        }

        // Render
        $this->gtk->show();
    }

    public function getPage(
        ?int $page_num = null
    ): ?\Yggverse\Yoda\Entity\Browser\Container\Page
    {
        // Get current page number on $page_num is null
        if (is_null($page_num))
        {
            $page_num = $this->gtk->get_current_page();

            // Return null if the notebook has no pages
            if ($page_num === -1)
            {
                return null;
            }
        }

        // Validate page index exists
        if (empty($this->_page[$page_num]))
        {
            throw new \Exception;
        }

        // Return page entity
        return $this->_page[$page_num];
    }

    public function closePage(
        ?int $page_num = null
    ): void
    {
        if ($page = $this->getPage($page_num))
        {
            $this->gtk->remove_page(
                $this->gtk->page_num(
                    $page->gtk
                )
            );
        }
    }

    public function reorderPage(
        ?int $page_num = null,
        bool $session = true
    ): void
    {
        // Reorder all pages
        if (is_null($page_num))
        {
            // Init new index
            $_page = [];

            foreach ($this->_page as $page)
            {
                // Get current entity $page_num
                $page_num = $this->gtk->page_num(
                    $page->gtk
                );

                // Skip deleted
                if ($page_num === -1)
                {
                    // Prevent session update
                    $session = false;

                    continue;
                }

                // Update position
                $_page[$page_num] = $page;
            }

            // Reorder entities
            $this->_page = $_page;

            // Update session
            if ($session)
            {
                $this->container->browser->database->cleanSession();

                ksort($_page);

                foreach ($_page as $page)
                {
                    $this->container->browser->database->addSession(
                        $page->navbar->request->getValue()
                    );
                }
            }
        }

        // Reorder by $page_num
        else throw new \Exception(
            'Reorder by $page_num value not implemented'
        );
    }
}