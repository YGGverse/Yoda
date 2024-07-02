<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Database
{
    private \PDO $_database;

    public function __construct(
         string $filename,
        ?string $username = null,
        ?string $password = null
    ) {
        // Init database
        $this->_database = new \PDO(
            sprintf(
                'sqlite:%s',
                $filename
            ),
            $username,
            $password
        );

        $this->_database->setAttribute(
            \PDO::ATTR_ERRMODE,
            \PDO::ERRMODE_EXCEPTION
        );

        $this->_database->setAttribute(
            \PDO::ATTR_DEFAULT_FETCH_MODE,
            \PDO::FETCH_OBJ
        );

        $this->_database->query('
            CREATE TABLE IF NOT EXISTS "history"
            (
                "id"    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "time"  INTEGER NOT NULL,
                "url"   VARCHAR(1024) NOT NULL,
                "title" VARCHAR(255)
            )
        ');
    }

    public function addHistory(
        string $url,
        ?string $title = null
    ): int
    {
        $query = $this->_database->prepare(
            'INSERT INTO `history` (`time`, `url`, `title`) VALUES (:time, :url, :title)'
        );

        $query->execute(
            [
                ':time'  => time(),
                ':url'   => $url,
                ':title' => $title
            ]
        );

        return (int) $this->_database->lastInsertId();
    }

    public function findHistory(
        string $value = '',
        int $start = 0,
        int $limit = 1000
    ): array
    {
        $query = $this->_database->prepare(
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

    public function deleteHistory(
        int $id
    ): int
    {
        $query = $this->_database->query(
            sprintf(
                'DELETE FROM `history` WHERE `id` = %d',
                $id
            )
        );

        return $query->rowCount();
    }

    public function cleanHistory(
        int $timeout = 0
    ): int
    {
        $query = $this->_database->query(
            sprintf(
                'DELETE FROM `history` WHERE `time` + %d < %d',
                $timeout,
                time()
            )

        );

        return $query->rowCount();
    }

    public function refreshHistory(
        string $url,
        ?string $title = null
    ): void
    {
        // Find same records match URL
        $query = $this->_database->prepare(
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
            $this->deleteHistory(
                $record->id
            );
        }

        // Add new record
        $this->addHistory(
            $url,
            $title
        );
    }
}