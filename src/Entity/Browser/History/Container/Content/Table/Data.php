<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\History\Container\Content\Table;

class Data
{
    public \GtkListStore $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\History\Container\Content\Table $table;

    // Defaults
    private string $_time = 'c';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\History\Container\Content\Table $table
    ) {
        // Init dependencies
        $this->table = $table;

        // Init tree view
        $this->gtk = new \GtkListStore(
            \GObject::TYPE_INT,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING,
            \GObject::TYPE_STRING
        );
    }

    // Append new row
    public function append(
        int $id,
        int $time,
        string $url,
        ?string $title
    ): void
    {
        $this->gtk->append(
            [
                $id,
                date(
                    $this->_time,
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
        $this->gtk->clear();
    }
}