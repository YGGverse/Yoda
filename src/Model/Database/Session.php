<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Database;

use \Pdo;

class Session
{
    public Pdo $connection;

    public function __construct(
        Pdo $connection
    ) {
        // Init parent connection
        $this->connection = $connection;

        // Init database structure
        $this->connection->query('
            CREATE TABLE IF NOT EXISTS `session`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` VARCHAR(1024)
            )
        ');
    }

    public function add(
        ?string $request = null,
        ?int $time = null
    ): int
    {
        $query = $this->connection->prepare(
            'INSERT INTO `session` (`time`, `request`) VALUES (:time, :request)'
        );

        $query->execute(
            [
                ':time'    => $time ? $time : time(),
                ':request' => $request
            ]
        );

        return intval(
            $this->connection->lastInsertId()
        );
    }

    public function get(): array
    {
        $query = $this->connection->query(
            'SELECT * FROM `session`'
        );

        if ($session = $query->fetchAll())
        {
            return $session;
        }

        return [];
    }

    public function clean(): int
    {
        $query = $this->connection->query(
            'DELETE FROM `session`'
        );

        return $query->rowCount();
    }
}