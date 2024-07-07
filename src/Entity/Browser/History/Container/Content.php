<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container;

use \Yggverse\Yoda\Entity\Browser\History\Container\Content\Viewport;
use \Yggverse\Yoda\Entity\Browser\History\Container\Content\Table;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\History\Container $container;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\History\Container\Content\Viewport $viewport;
    public \Yggverse\Yoda\Entity\Browser\History\Container\Content\Table $table;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\History\Container $container
    ) {
        // Init dependency
        $this->container = $container;

        // Init container
        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        // Init history records table
        $this->table = new Table(
            $this
        );

        // Init viewport to integrate scrolled window features
        $this->viewport = new Viewport(
            $this
        );

        $this->viewport->gtk->add(
            $this->table->gtk
        );

        $this->gtk->add(
            $this->viewport->gtk
        );

        // Do initial search
        $this->search();
    }

    // Do records search in database
    public function search(
        string $filter = ''
    ): void
    {
        $this->table->data->clear();

        if ($records = $this->container->history->browser->database->findHistory($filter))
        {
            foreach ($records as $record)
            {
                $this->table->data->append(
                    $record->id,
                    $record->time,
                    $record->url,
                    $record->title
                );
            }
        }

        else
        {
            $this->container->navbar->open->gtk->set_sensitive(
                false
            );

            $this->container->navbar->delete->gtk->set_sensitive(
                false
            );
        }
    }

    // Refresh rows using current filter value in the navbar
    public function refresh(): void
    {
        $this->search(
            $this->container->navbar->filter->gtk->get_text()
        );
    }
}