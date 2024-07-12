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
        $this->append(
            'gemini://yggverse.cities.yesterweb.org'
        );

        $this->append(
            'gemini://tlgs.one'
        );

        $this->append(
            'nex://nightfall.city'
        );

        // Init events
        $this->gtk->connect(
            'switch-page',
            function (
                \GtkNotebook $entity,
                \GtkWidget $child,
                int $index
            ) {
                // Update header bar title
                $this->container->browser->header->setTitle(
                    $this->getPage($index)->title->getValue(),
                    $this->getPage($index)->title->getSubtitle()
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
        $page = new Page(
            $this->container
        );

        if ($request)
        {
            $page->open(
                $request
            );
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
                $page->title->getValue(),
                $page->title->getSubtitle()
            );
        }

        // Render
        $this->gtk->show();

        // Extendable classes not supported by PHP-GTK3 #117
        // create internal pages registry
        $this->_page[] = $page;
    }

    public function getPage(
        int $index
    ): ?\Yggverse\Yoda\Entity\Browser\Container\Page
    {
        if (empty($this->_page[$index]))
        {
            throw new \Exception;
        }

        return $this->_page[$index];
    }
}