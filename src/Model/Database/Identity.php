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
                `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time` INTEGER NOT NULL,
                `name` VARCHAR(255),
                `crt`  TEXT NOT NULL,
                `key`  TEXT NOT NULL
            )
        ');
    }

    public function add(
        ?string $crt,
        ?string $key,
        ?string $name = null,
           ?int $time = null
    ): int
    {
        $query = $this->_connection->prepare(
            'INSERT INTO `identity` (
                `time`,
                `name`,
                `crt`,
                `key`
            ) VALUES (
                :time,
                :name,
                :crt,
                :key
            )'
        );

        $query->execute(
            [
                ':time' => $time ? $time : time(),
                ':name' => $name,
                ':crt'  => $crt,
                ':key'  => $key
            ]
        );

        return intval(
            $this->_connection->lastInsertId()
        );
    }

    public function delete(
        int $id
    ): int
    {
        $query = $this->_connection->prepare(
            'DELETE FROM `identity` WHERE `id` = :id'
        );

        $query->execute(
            [
                ':id' => $id
            ]
        );

        return $query->rowCount();
    }

    public function get(
        int $id
    ): ?object
    {
        $query = $this->_connection->prepare(
            'SELECT * FROM `identity` WHERE `id` = :id'
        );

        $query->execute(
            [
                ':id' => $id
            ]
        );

        if ($identity = $query->fetch())
        {
            return $identity;
        }

        return null;
    }

    public function find(
        string $name  = '',
           int $start = 0,
           int $limit = 1000
    ): array
    {
        $query = $this->_connection->prepare(
            sprintf(
                'SELECT * FROM `identity`
                          WHERE `name` LIKE :name
                          ORDER BY `name` ASC
                          LIMIT %d,%d',
                $start,
                $limit
            )
        );

        $query->execute(
            [
                ':name' => sprintf(
                    '%%%s%%',
                    $name
                )
            ]
        );

        return $query->fetchAll();
    }
}