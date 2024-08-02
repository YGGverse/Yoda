<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

use \Exception;
use \Gdk;
use \GdkEvent;
use \GtkEventBox;
use \GtkNotebook;
use \GtkWidget;

use \Yggverse\Yoda\Entity\Browser\Container;

class Tab
{
    // GTK
    public GtkNotebook $gtk;

    // Dependencies
    public Container $container;

    // Defaults
    public const REORDERABLE = true;
    public const SCROLLABLE  = true;

    // Extras
    public array $pages = [];

    public function __construct(
        Container $container
    ) {
        // Init dependency
        $this->container = $container;

        // Init container
        $this->gtk = new GtkNotebook;

        $this->gtk->set_scrollable(
            $this::SCROLLABLE
        );

        // Restore previous session
        foreach ($this->container->browser->database->session->get() as $session)
        {
            $this->append(
                $session->request,
                boolval( // open
                    parse_url(
                        $session->request,
                        PHP_URL_SCHEME
                    )
                )
            );
        }

        // Init events
        $this->gtk->connect(
            'switch-page',
            function(
                GtkNotebook $self,
                GtkWidget $child,
                int $page_num
            ) {
                // Update header bar title
                if ($page = $this->get($page_num))
                {
                    $this->container->browser->header->setTitle(
                        $page->title->getValue(),
                        $page->title->getSubtitle()
                    );

                } else throw new Exception;

                // Keep current selection
                $self->grab_focus();
            }
        );

        $this->gtk->connect(
            'page-added',
            function(
                GtkNotebook $self,
                GtkWidget $child,
                int $page_num
            ) {
                $this->reorder();
            }
        );

        $this->gtk->connect(
            'page-removed',
            function(
                GtkNotebook $self,
                GtkWidget $child,
                int $page_num
            ) {
                // Free memory pool
                if ($page = $this->get($page_num))
                {
                    if ($page->connection)
                    {
                        $page->connection->close();
                    }

                } else throw new Exception;

                // Reorder pages
                $this->reorder();
            }
        );

        $this->gtk->connect(
            'page-reordered',
            function(
                GtkNotebook $self,
                GtkWidget $child,
                int $page_num
            ) {
                $this->reorder();
            }
        );
    }

    public function append(
        ?string $request = null,
        bool $open = true,
        bool $focus = true
    ): void
    {
        // Extendable classes not supported by PHP-GTK3 #117
        // create internal pages registry
        $this->pages[] = $page = new Page(
            $this->container
        );

        // Create event box to listen for double click on title label
        $label = new GtkEventBox;

        $label->add(
            $page->title->gtk
        );

        $label->show_all();

        $label->connect(
            'button-press-event',
            function(
                GtkEventBox $self,
                GdkEvent $event
            ) {
                // Close tab on double click
                if ($event->type == Gdk::DOUBLE_BUTTON_PRESS)
                {
                    $this->close();
                }
            }
        );

        $this->gtk->append_page(
            $page->gtk,
            $label
        );

        $this->gtk->set_tab_reorderable(
            $page->gtk,
            $this::REORDERABLE
        );

        if ($open)
        {
            $page->open(
                $request
            );
        }

        else
        {
            $page->init(
                $request,
                empty(
                    $request
                )
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

    public function get(
        ?int $page_num = null
    ): ?Page
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
        if (empty($this->pages[$page_num]))
        {
            throw new Exception;
        }

        // Return page entity
        return $this->pages[$page_num];
    }

    public function close(
        ?int $page_num = null
    ): void
    {
        if ($page = $this->get($page_num))
        {
            $this->gtk->remove_page(
                $this->gtk->page_num(
                    $page->gtk
                )
            );

            $this->reorder();
        }
    }

    public function clean(): void
    {
        while ($this->pages)
        {
            $this->close();
        }
    }

    public function reorder(): void
    {
        // Init new index
        $pages = [];

        foreach ($this->pages as $page)
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
            $pages[$page_num] = $page;
        }

        // Reorder entities
        $this->pages = $pages;

        ksort(
            $this->pages
        );
    }
}