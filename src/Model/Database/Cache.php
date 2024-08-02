<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Database;

use \Pdo;

class Cache
{
    protected Pdo $_connection;

    public function __construct(
        Pdo $connection
    ) {
        // Init parent connection
        $this->_connection = $connection;

        // Init database structure
        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `cache`
            (
                `id`       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`     INTEGER NOT NULL,
                `request`  VARCHAR(1024) UNIQUE,
                `mime`     VARCHAR(255),
                `title`    VARCHAR(255),
                `subtitle` VARCHAR(255),
                `tooltip`  VARCHAR(255),
                `data`     BLOB
            )
        ');
    }

    public function add(
        ?string $request = null,
        ?string $mime = null,
        ?string $title = null,
        ?string $subtitle = null,
        ?string $tooltip = null,
        ?string $data = null,
        ?int $time = null
    ): int
    {
        $query = $this->_connection->prepare(
            'INSERT INTO `cache` (
                `time`,
                `request`,
                `mime`,
                `title`,
                `subtitle`,
                `tooltip`,
                `data`
            ) VALUES (
                :time,
                :request,
                :mime,
                :title,
                :subtitle,
                :tooltip,
                :data
            )'
        );

        $query->execute(
            [
                ':time'     => $time ? $time : time(),
                ':request'  => $request,
                ':mime'     => $mime,
                ':title'    => $title,
                ':subtitle' => $subtitle,
                ':tooltip'  => $tooltip,
                ':data'     => $data
            ]
        );

        return intval(
            $this->_connection->lastInsertId()
        );
    }

    public function get(
        string $request = ''
    ): ?object
    {
        $query = $this->_connection->prepare(
            'SELECT * FROM `cache` WHERE `request` LIKE :request'
        );

        $query->execute(
            [
                ':request' => $request
            ]
        );

        if ($cache = $query->fetch())
        {
            return $cache;
        }

        return null;
    }

    public function delete(
        int $id
    ): int
    {
        $query = $this->_connection->query(
            sprintf(
                'DELETE FROM `cache` WHERE `id` = %d',
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
                'DELETE FROM `cache` WHERE `time` + %d < %d',
                $timeout,
                time()
            )
        );

        return $query->rowCount();
    }

    public function renew(
        string $request,
        ?string $mime = null,
        ?string $title = null,
        ?string $subtitle = null,
        ?string $tooltip = null,
        ?string $data = null,
        ?int $time = null
    ): void
    {
        // Find same records match URL
        $query = $this->_connection->prepare(
            'SELECT * FROM `cache` WHERE `request` LIKE :request'
        );

        $query->execute(
            [
                ':request' => $request
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
            $request,
            $mime,
            $title,
            $subtitle,
            $tooltip,
            $data,
            $time
        );
    }
}