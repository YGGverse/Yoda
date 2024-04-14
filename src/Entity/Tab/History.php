<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Tab;

class History
{
    public \Yggverse\Yoda\Entity\App $app;

    public \GtkBox $box,
                   $header,
                   $body;

    public \GtkButton $open,
                      $clear,
                      $go;

    public \GtkEntry $search;

    public \GtkListStore $list;
    public \GtkTreeView $treeview;
    public \GtkScrolledWindow $container;

    public object $config;

    public function __construct(
        \Yggverse\Yoda\Entity\App $app
    ) {
        // Init app
        $this->app = $app;

        // Init config
        $this->config = \Yggverse\Yoda\Model\File::getConfig()->app->tab->history;

        // Compose header
        $this->header = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->header->set_margin_top(
            $this->config->header->margin
        );

        $this->header->set_margin_bottom(
            $this->config->header->margin
        );

        $this->header->set_margin_start(
            $this->config->header->margin
        );

        $this->header->set_margin_end(
            $this->config->header->margin
        );

        $this->header->set_spacing(
            $this->config->header->margin
        );

        // Open button
        $this->open = \GtkButton::new_with_label(
            $this->config->header->button->open->label
        );

        $this->open->set_sensitive(
            false
        );

        $this->open->connect(
            'clicked',
            function ()
            {
                // @TODO

                $this->refresh();
            }
        );

        if ($this->config->header->button->open->visible)
        {
            $this->header->add(
                $this->open
            );
        }

        // Clear button
        $this->clear = \GtkButton::new_with_label(
            $this->config->header->button->clear->label
        );

        $this->clear->set_sensitive(
            false
        );

        $this->clear->connect(
            'clicked',
            function ()
            {
                // @TODO

                $this->refresh();
            }
        );

        if ($this->config->header->button->clear->visible)
        {
            $this->header->add(
                $this->clear
            );
        }

        // Search field
        $this->search = new \GtkEntry;

        $this->search->set_placeholder_text(
            $this->config->header->search->placeholder
        );

        $this->search->connect(
            'activate',
            function ($entry)
            {
                $this->refresh(
                    $entry->get_text()
                );
            }
        );

        $this->header->pack_start(
            $this->search,
            true,
            true,
            0
        );

        // Go button
        $this->go = \GtkButton::new_with_label(
            $this->config->header->button->go->label
        );

        $this->go->connect(
            'clicked',
            function ()
            {
                $this->refresh(
                    $this->search->get_text()
                );
            }
        );

        if ($this->config->header->button->go->visible)
        {
            $this->header->add(
                $this->go
            );
        }

        // Build history list
        $this->treeview = new \GtkTreeView();

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'Time',
                new \GtkCellRendererText(),
                'text',
                0
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'Title',
                new \GtkCellRendererText(),
                'text',
                1
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'URL',
                new \GtkCellRendererText(),
                'text',
                2
            )
        );

        // Init list storage
        $this->list = new \GtkListStore(
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING
        );

        $this->treeview->set_model(
            $this->list
        );

        $this->treeview->get_selection()->set_mode(
            \GtkSelectionMode::MULTIPLE
        );

        // Compose body
        $this->body = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->container = new \GtkScrolledWindow();

        $this->container->add(
            $this->treeview
        );

        $this->body->set_margin_start(
            $this->config->body->margin
        );

        $this->body->pack_start(
            $this->container,
            true,
            true,
            0
        );

        // Compose page
        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->box->add(
            $this->header
        );

        $this->box->pack_start(
            $this->body,
            true,
            true,
            0
        );

        // Refresh history
        $this->refresh();

        // Activate events
        $this->treeview->connect(
            'row-activated',
            function ()
            {
                // @TODO
            }
        );
    }

    public function refresh(
        string $search = ''
    ): void
    {
        // Reset previous state
        $this->list->clear();

        // Update buttons sensibility
        $this->open->set_sensitive(
            false
        );

        $this->clear->set_sensitive(
            false
        );

        // Build history list from database records
        foreach ($this->app->database->getHistory($search) as $record)
        {
            $this->list->append(
                [
                    date(
                        $this->config->time->format,
                        $record->time
                    ),
                    $record->title,
                    $record->url
                ]
            );
        }
    }
}