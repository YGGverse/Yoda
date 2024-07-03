<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\History;

use \Yggverse\Yoda\Entity\Window\Tab\Address;

class Content
{
    public \GtkScrolledWindow $gtk;

    public \GtkTreeView $treeview;
    public \GtkListStore $list;

    public \Yggverse\Yoda\Entity\Window\Tab\History $history;

    // Defaults
    private string $_time   = 'Time';
    private string $_title  = 'Title';
    private string $_url    = 'URL';
    private string $_format = 'c';
    private int    $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\History $history
    ) {
        $this->history = $history;

        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->treeview = new \GtkTreeView;

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

        $this->search();

        $this->treeview->connect(
            'row-activated',
            function(
                \GtkTreeView $treeview
            ) {
                $address = new Address(
                    $this->history->tab
                );

                $address->navbar->request->gtk->set_text(
                    $this->getSelectedUrl()
                );

                $this->history->tab->append(
                    $address
                );

                $address->update();
            }
        );

        $this->treeview->connect(
            'cursor-changed',
            function(
                \GtkTreeView $treeview
            ) {
                $this->history->navbar->open->gtk->set_sensitive(
                    (bool) $this->getSelectedId()
                );

                $this->history->navbar->delete->gtk->set_sensitive(
                    (bool) $this->getSelectedId()
                );
            }
        );
    }

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

    public function clear(): void
    {
        $this->list->clear();
    }

    public function search(
        string $filter = ''
    ): void
    {
        $this->clear();

        if ($records = $this->history->tab->window->database->findHistory($filter))
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
            /* @TODO initiation test
            $this->history->navbar->open->gtk->set_sensitive(
                false
            );

            $this->history->navbar->delete->gtk->set_sensitive(
                false
            );
            */
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