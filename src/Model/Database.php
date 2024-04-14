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
                CREATE TABLE IF NOT EXISTS "pageHistory"
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

    public function addPageHistory(
        string $url
    ): int
    {
        $query = $this->_database->prepare(
            'INSERT INTO `pageHistory` (`time`, `url`) VALUES (:time, :url)'
        );

        $query->execute(
            [
                ':time' => time(),
                ':url'  => $url
            ]
        );

        return (int) $this->_database->lastInsertId();
    }

    public function getPageHistory(
        int $start = 0,
        int $limit = 1000
    ): array
    {
        $query = $this->_database->query(
            sprintf(
                'SELECT * FROM `pageHistory` ORDER BY `id` DESC LIMIT %d,%d',
                $start,
                $limit
            )

        );

        return $query->fetchAll();
    }

    public function cleanPageHistory(
        int $timeout = 0
    ): int
    {
        $query = $this->_database->query(
            sprintf(
                'DELETE FROM `pageHistory` WHERE `time` + %d < %d',
                $timeout,
                time()
            )

        );

        return $query->rowCount();
    }
}