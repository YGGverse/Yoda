<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Requirements @TODO entity
    public \GtkTreeView $treeview;
    public \GtkListStore $list;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\History\Container $container;

    // Defaults
    private string $_time   = 'Time';
    private string $_title  = 'Title';
    private string $_url    = 'URL';
    private string $_format = 'c';
    private int    $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\History\Container $container
    ) {
        // Init dependency
        $this->container = $container;

        // Init container
        $this->gtk = new \GtkScrolledWindow;

        // Init records listing
        $this->treeview = new \GtkTreeView;

        $this->treeview->set_margin_start(
            $this->_margin
        );

        $this->treeview->set_margin_end(
            $this->_margin
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                $this->_time,
                new \GtkCellRendererText(),
                'text',
                1
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                $this->_url,
                new \GtkCellRendererText(),
                'text',
                2
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                $this->_title,
                new \GtkCellRendererText(),
                'text',
                3
            )
        );

        $this->list = new \GtkListStore(
            \GObject::TYPE_INT,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING
        );

        $this->treeview->set_model(
            $this->list
        );

        $this->gtk->add(
            $this->treeview
        );

        // Connect events
        $this->treeview->connect(
            'row-activated',
            function(
                \GtkTreeView $treeview
            ) {
                // Focus on browser
                // $this->container->history->browser->gtk->present();

                $this->container->history->browser->container->tab->append(
                    $this->getSelectedUrl()
                );
            }
        );

        $this->treeview->connect(
            'cursor-changed',
            function(
                \GtkTreeView $treeview
            ) {
                $this->container->navbar->open->gtk->set_sensitive(
                    boolval(
                        $this->getSelectedId()
                    )
                );

                $this->container->navbar->delete->gtk->set_sensitive(
                    boolval(
                        $this->getSelectedId()
                    )
                );
            }
        );

        // Make initial search
        $this->search();
    }

    // Append new row
    public function append(
        int $id,
        int $time,
        string $url,
        ?string $title
    ): void
    {
        $this->list->append(
            [
                $id,
                date(
                    $this->_format,
                    $time
                ),
                $url,
                strval(
                    $title
                )
            ]
        );
    }

    // Remove rows from list
    public function clear(): void
    {
        $this->list->clear();
    }

    // Refresh rows using current navbar value
    public function refresh(): void
    {
        $this->search(
            $this->container->navbar->filter->gtk->get_text()
        );
    }

    // Do records search in database
    public function search(
        string $filter = ''
    ): void
    {
        $this->clear();

        if ($records = $this->container->history->browser->database->findHistory($filter))
        {
            foreach ($records as $record)
            {
                $this->append(
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

    public function getSelectedId(): ?int
    {
        if ($id = $this->_getSelected(0))
        {
            return $id;
        }

        return null;
    }

    public function getSelectedUrl(): ?string
    {
        if ($url = $this->_getSelected(2))
        {
            return $url;
        }

        return null;
    }

    private function _getSelected(
        int $column
    ): null|int|string
    {
        list(
            $list,
            $row
        ) = $this->treeview->get_selection()->get_selected();

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