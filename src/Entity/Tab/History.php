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
                      $delete,
                      $search;

    public \GtkEntry $filter;

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

        if ($this->config->header->button->open->visible)
        {
            $this->header->add(
                $this->open
            );
        }

        // Delete button
        $this->delete = \GtkButton::new_with_label(
            $this->config->header->button->delete->label
        );

        $this->delete->set_sensitive(
            false
        );

        if ($this->config->header->button->delete->visible)
        {
            $this->header->add(
                $this->delete
            );
        }

        // Filter field
        $this->filter = new \GtkEntry;

        $this->filter->set_placeholder_text(
            $this->config->header->filter->placeholder
        );

        $this->header->pack_start(
            $this->filter,
            true,
            true,
            0
        );

        // Search button
        $this->search = \GtkButton::new_with_label(
            $this->config->header->button->search->label
        );

        if ($this->config->header->button->search->visible)
        {
            $this->header->add(
                $this->search
            );
        }

        // Build history list
        $this->treeview = new \GtkTreeView();

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'Time',
                new \GtkCellRendererText(),
                'text',
                1
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'Title',
                new \GtkCellRendererText(),
                'text',
                2
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'URL',
                new \GtkCellRendererText(),
                'text',
                3
            )
        );

        // Init list storage
        $this->list = new \GtkListStore(
            \GObject::TYPE_INT,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING
        );

        $this->treeview->set_model(
            $this->list
        );

        /* @TODO row-activated
        $this->treeview->get_selection()->set_mode(
            \GtkSelectionMode::MULTIPLE
        );
        */

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
            function ($tree)
            {
                if ($url = $this->getSelectedColumn(3, $tree))
                {
                    $page = $this->app->blankPage();

                    $page->open(
                        $url
                    );
                }
            }
        );

        $this->treeview->connect(
            'cursor-changed',
            function ($tree)
            {
                $url = $this->getSelectedColumn(
                    3, $tree
                );

                $this->open->set_sensitive(
                    (bool) $url
                );

                $this->delete->set_sensitive(
                    (bool) $url
                );
            }
        );

        $this->filter->connect(
            'activate',
            function ($entry)
            {
                $this->refresh(
                    $entry->get_text()
                );
            }
        );

        if ($this->config->header->button->open->visible)
        {
            $this->open->connect(
                'clicked',
                function ()
                {
                    if ($url = $this->getSelectedColumn(3))
                    {
                        $page = $this->app->blankPage();

                        $page->open(
                            $url
                        );

                        $this->refresh();
                    }
                }
            );
        }

        if ($this->config->header->button->delete->visible)
        {
            $this->delete->connect(
                'clicked',
                function ()
                {
                    if ($id = $this->getSelectedColumn(0))
                    {
                        $this->app->database->deleteHistory(
                            $id
                        );

                        $this->refresh();
                    }
                }
            );
        }

        if ($this->config->header->button->search->visible)
        {
            $this->search->connect(
                'clicked',
                function ()
                {
                    $this->refresh(
                        $this->filter->get_text()
                    );
                }
            );
        }
    }

    public function refresh(): void
    {
        // Reset previous state
        $this->list->clear();

        // Update buttons sensibility
        $this->open->set_sensitive(
            false
        );

        $this->delete->set_sensitive(
            false
        );

        // Build history list from database records
        foreach ($this->app->database->getHistory($this->filter->get_text()) as $record)
        {
            $this->list->append(
                [
                    $record->id,
                    date(
                        $this->config->time->format,
                        $record->time
                    ),
                    $record->title,
                    $record->url
                ]
            );
        }

        // Update tree
        $this->treeview->show_all();
    }

    public function getSelectedColumn(
        int $column,
        \GtkTreeView $treeview = null
    ): null|int|string
    {
        if (is_null($treeview))
        {
            $treeview = $this->treeview;
        }

        list(
            $list,
            $row
        ) = $treeview->get_selection()->get_selected();

        if ($list && $row)
        {
            if ($value = $list->get_value($row, $column))
            {
                return $value;
            }
        }

        return null;
    }
}