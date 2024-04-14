<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Database
{
    public \PDO $_database;

    public function __construct(
        string  $database,
        ?string $username = null,
        ?string $password = null
    ) {
        try
        {
            $this->_database = new \PDO(
                sprintf(
                    'sqlite:%s',
                    $database
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
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    "time" INTEGER NOT NULL,
                    "url" VARCHAR(1024) NOT NULL
                )
            ');
        }

        catch (\PDOException $exception)
        {
            exit(
                print_r(
                    $exception->getMessage(),
                    true
                )
            );
        }
    }

    public function addHistory(
        string $url
    ): int
    {
        $query = $this->_database->prepare(
            'INSERT INTO `history` (`time`, `url`) VALUES (:time, :url)'
        );

        $query->execute(
            [
                ':time' => time(),
                ':url'  => $url
            ]
        );

        return (int) $this->_database->lastInsertId();
    }

    public function getHistory(
        string $search = '',
        int $start = 0,
        int $limit = 1000
    ): array
    {
        $query = $this->_database->prepare(
            sprintf(
                'SELECT * FROM `history` WHERE `url` LIKE :search ORDER BY `id` DESC LIMIT %d,%d',
                $start,
                $limit
            )
        );

        $query->execute(
            [
                ':search' => sprintf(
                    '%%%s%%',
                    $search
                )
            ]
        );

        return $query->fetchAll();
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
}