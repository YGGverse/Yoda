<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Database;

use \Pdo;

class Auth
{
    protected Pdo $_connection;

    public function __construct(
        Pdo $connection
    ) {
        // Init parent connection
        $this->_connection = $connection;

        // Init database structure
        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `auth`
            (
                `id`       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`     INTEGER NOT NULL,
                `active`   INTEGER NOT NULL,
                `identity` INTEGER NOT NULL,
                `request`  VARCHAR(1024) NOT NULL
            )
        ');
    }
}