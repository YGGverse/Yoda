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
                `identity` INTEGER NOT NULL,
                `request`  VARCHAR(1024) NOT NULL
            )
        ');
    }

    public function add(
        int $identity,
        string $request,
        ?int $time = null
    ): int
    {
        $query = $this->_connection->prepare(
            'INSERT INTO `auth` (
                `time`,
                `identity`,
                `request`
            ) VALUES (
                :time,
                :identity,
                :request
            )'
        );

        $query->execute(
            [
                ':time'     => $time ? $time : time(),
                ':identity' => $identity,
                ':request'  => $request
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
            'DELETE FROM `auth` WHERE `id` = :id'
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
            'SELECT * FROM `auth` WHERE `id` = :id'
        );

        $query->execute(
            [
                ':id' => $id
            ]
        );

        if ($record = $query->fetch())
        {
            return $record;
        }

        return null;
    }

    public function match(
        string $request = '',
        int $limit = 1000,
        int $start = 0
    ): array
    {
        $query = $this->_connection->prepare(
            sprintf(
                'SELECT * FROM `auth`
                          WHERE `request` LIKE :request
                          ORDER BY `request` ASC
                          LIMIT %d,%d',
                $start,
                $limit
            )
        );

        $query->execute(
            [
                ':request' => $request
            ]
        );

        return $query->fetchAll();
    }

    public function logout(
        string $request
    ): int
    {
        $records = 0;

        foreach ($this->match($request) as $record)
        {
            $records += $this->delete(
                $record->id
            );
        }

        return $records;
    }
}