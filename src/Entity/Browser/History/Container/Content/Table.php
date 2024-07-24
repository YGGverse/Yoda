<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Content;

use \GtkCellRendererText;
use \GtkTreeView;
use \GtkTreeViewColumn;

use \Yggverse\Yoda\Entity\Browser\History\Container\Content;

class Table
{
    public GtkTreeView $gtk;

    // Dependencies
    public Content $content;

    // Requirements
    public Table\Data $data;

    // Defaults
    public const TIME = 'Time';
    public const TITLE = 'Title';
    public const URL = 'URL';

    public function __construct(
        Content $content
    ) {
        // Init dependencies
        $this->content = $content;

        // Init tree view
        $this->gtk = new GtkTreeView;

        $this->gtk->append_column(
            new GtkTreeViewColumn(
                $this::TIME,
                new GtkCellRendererText(),
                'text',
                1
            )
        );

        $this->gtk->append_column(
            new GtkTreeViewColumn(
                $this::URL,
                new GtkCellRendererText(),
                'text',
                2
            )
        );

        $this->gtk->append_column(
            new GtkTreeViewColumn(
                $this::TITLE,
                new GtkCellRendererText(),
                'text',
                3
            )
        );

        // Init data model
        $this->data = new Table\Data(
            $this
        );

        $this->gtk->set_model(
            $this->data->gtk
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'row-activated',
            function()
            {
                // Focus on browser
                // $this->content->container->history->browser->gtk->present();

                $this->content->container->history->browser->container->tab->append(
                    $this->getSelectedUrl()
                );
            }
        );

        $this->gtk->connect(
            'cursor-changed',
            function()
            {
                $this->content->container->navbar->open->gtk->set_sensitive(
                    boolval(
                        $this->getSelectedId()
                    )
                );

                $this->content->container->navbar->delete->gtk->set_sensitive(
                    boolval(
                        $this->getSelectedId()
                    )
                );
            }
        );
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
        ) = $this->gtk->get_selection()->get_selected();

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