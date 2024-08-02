<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Database;

use \Pdo;

class Bookmark
{
    public Pdo $connection;

    public function __construct(
        Pdo $connection
    ) {
        // Init parent connection
        $this->connection = $connection;

        // Init database structure
        $this->connection->query('
            CREATE TABLE IF NOT EXISTS `bookmark`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` VARCHAR(1024) UNIQUE,
                `title`   VARCHAR(255)
            )
        ');
    }

    public function add(
        ?string $request = null,
        ?string $title = null,
        ?int $time = null
    ): int
    {
        $query = $this->connection->prepare(
            'INSERT INTO `bookmark` (
                `time`,
                `request`,
                `title`
            ) VALUES (
                :time,
                :request,
                :title
            )'
        );

        $query->execute(
            [
                ':time'    => $time ? $time : time(),
                ':request' => $request,
                ':title'   => $title
            ]
        );

        return intval(
            $this->connection->lastInsertId()
        );
    }

    public function get(
        ?string $request = null
    ): ?object
    {
        $query = $this->connection->prepare(
            'SELECT * FROM `bookmark` WHERE `request` LIKE :request'
        );

        $query->execute(
            [
                ':request' => $request
            ]
        );

        if ($record = $query->fetch())
        {
            return $record;
        }

        return null;
    }

    public function find(
        ?string $value = null,
        int $start = 0,
        int $limit = 1000
    ): array
    {
        $query = $this->connection->prepare(
            sprintf(
                'SELECT * FROM `bookmark`
                          WHERE `request` LIKE :value OR `title` LIKE :value
                          ORDER BY `id` DESC
                          LIMIT %d,%d',
                $start,
                $limit
            )
        );

        $query->execute(
            [
                ':value' => sprintf(
                    '%%%s%%',
                    strval(
                        $value
                    )
                )
            ]
        );

        return $query->fetchAll();
    }

    public function delete(
        int $id
    ): int
    {
        $query = $this->connection->query(
            sprintf(
                'DELETE FROM `bookmark` WHERE `id` = %d',
                $id
            )
        );

        return $query->rowCount();
    }

    public function toggle(
        ?string $request = null,
        ?string $title = null,
        ?int $time = null
    ): bool
    {
        if ($record = $this->get($request))
        {
            $this->delete(
                $record->id
            );

            return false;
        }

        else
        {
            $this->add(
                $request,
                $title,
                $time
            );

            return true;
        }
    }
}