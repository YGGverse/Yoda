<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Tab;

class History
{
    public \Yggverse\Yoda\Entity\App $app;

    public \GtkBox $box;
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

        // Build history list
        $this->treeview = new \GtkTreeView();

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'URL',
                new \GtkCellRendererText(),
                'text',
                0
            )
        );

        $this->treeview->append_column(
            new \GtkTreeViewColumn(
                'Time',
                new \GtkCellRendererText(),
                'text',
                1
            )
        );

        // Create list storage
        $this->list = new \GtkListStore(
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING
        );

        $this->treeview->set_model(
            $this->list
        );

        // Build history list from database records
        foreach ($this->app->database->getHistory() as $record)
        {
            $this->list->append(
                [
                    $record->url,
                    date(
                        $this->config->time->format,
                        $record->time
                    )
                ]
            );
        }

        // Compose page
        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->container = new \GtkScrolledWindow();

        $this->container->add(
            $this->treeview
        );

        $this->box->pack_start(
            $this->container,
            true,
            true,
            0
        );
    }
}