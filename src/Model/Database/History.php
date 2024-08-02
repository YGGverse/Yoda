<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Database;

use \Pdo;

class History
{
    protected Pdo $_connection;

    public function __construct(
        Pdo $connection
    ) {
        // Init parent connection
        $this->_connection = $connection;

        // Init database structure
        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `history`
            (
                `id`    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`  INTEGER NOT NULL,
                `url`   VARCHAR(1024) NOT NULL,
                `title` VARCHAR(255)
            )
        ');
    }

    public function add(
        string $url,
        ?string $title = null
    ): int
    {
        $query = $this->_connection->prepare(
            'INSERT INTO `history` (`time`, `url`, `title`) VALUES (:time, :url, :title)'
        );

        $query->execute(
            [
                ':time'  => time(),
                ':url'   => $url,
                ':title' => $title
            ]
        );

        return intval(
            $this->_connection->lastInsertId()
        );
    }

    public function find(
        string $value = '',
        int $start = 0,
        int $limit = 1000
    ): array
    {
        $query = $this->_connection->prepare(
            sprintf(
                'SELECT * FROM `history`
                          WHERE `url` LIKE :value OR `title` LIKE :value
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
                    $value
                )
            ]
        );

        return $query->fetchAll();
    }

    public function delete(
        int $id
    ): int
    {
        $query = $this->_connection->query(
            sprintf(
                'DELETE FROM `history` WHERE `id` = %d',
                $id
            )
        );

        return $query->rowCount();
    }

    public function clean(
        int $timeout = 0
    ): int
    {
        $query = $this->_connection->query(
            sprintf(
                'DELETE FROM `history` WHERE `time` + %d < %d',
                $timeout,
                time()
            )

        );

        return $query->rowCount();
    }

    public function renew(
        string $url,
        ?string $title = null
    ): void
    {
        // Find same records match URL
        $query = $this->_connection->prepare(
            'SELECT * FROM `history` WHERE `url` LIKE :url'
        );

        $query->execute(
            [
                ':url' => $url
            ]
        );

        // Drop previous records
        foreach ($query->fetchAll() as $record)
        {
            $this->delete(
                $record->id
            );
        }

        // Add new record
        $this->add(
            $url,
            $title
        );
    }
}