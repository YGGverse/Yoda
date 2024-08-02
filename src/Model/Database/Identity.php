<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Database;

use \Pdo;

class Identity
{
    protected Pdo $_connection;

    public function __construct(
        Pdo $connection
    ) {
        // Init parent connection
        $this->_connection = $connection;

        // Init database structure
        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `identity`
            (
                `id`     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`   INTEGER NOT NULL,
                `active` INTEGER NOT NULL,
                `name`   VARCHAR(255),
                `crt`    TEXT NOT NULL,
                `key`    TEXT NOT NULL
            )
        ');
    }
}