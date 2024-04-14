<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Database
{
    public \PDO $database;

    public function __construct(
        string $database,
        ?string $username = null,
        ?string $password = null
    ) {
        try
        {
            $this->database = new \PDO(
                sprintf(
                    'sqlite:%s',
                    $database
                ),
                $username,
                $password
            );

            $this->database->setAttribute(
                \PDO::ATTR_ERRMODE,
                \PDO::ERRMODE_EXCEPTION
            );

            $this->database->setAttribute(
                \PDO::ATTR_DEFAULT_FETCH_MODE,
                \PDO::FETCH_OBJ
            );

            $this->database->query('
                CREATE TABLE IF NOT EXISTS "history"
                (
                    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    "time" INTEGER NOT NULL,
                    "address" VARCHAR(1024) NOT NULL
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
}