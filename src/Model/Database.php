<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Database
{
    private \PDO $_connection;

    private bool $_exists;

    public function __construct(
         string $filename,
        ?string $username = null,
        ?string $password = null
    ) {
        // Status
        $this->_exists = file_exists(
            $filename
        );

        // Init database connection
        $this->_connection = new \PDO(
            sprintf(
                'sqlite:%s',
                $filename
            ),
            $username,
            $password
        );

        $this->_connection->setAttribute(
            \PDO::ATTR_ERRMODE,
            \PDO::ERRMODE_EXCEPTION
        );

        $this->_connection->setAttribute(
            \PDO::ATTR_DEFAULT_FETCH_MODE,
            \PDO::FETCH_OBJ
        );

        // Init tables
        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `bookmark`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` VARCHAR(1024) UNIQUE,
                `title`   VARCHAR(255)
            )
        ');

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
            );
        ');

        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `history`
            (
                `id`    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`  INTEGER NOT NULL,
                `url`   VARCHAR(1024) NOT NULL,
                `title` VARCHAR(255)
            )
        ');

        $this->_connection->query('
            CREATE TABLE IF NOT EXISTS `session`
            (
                `id`      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                `time`    INTEGER NOT NULL,
                `request` VARCHAR(1024) UNIQUE
            );
        ');

        // Initial setup
        if (!$this->_exists)
        {
            // Init welcome page
            $this->addSession(
                'gemini://yggverse.cities.yesterweb.org' // @TODO config
            );
        }
    }

    // Bookmark
    public function addBookmark(
        ?string $request = null,
        ?string $title = null,
        ?int $time = null
    ): int
    {
        $query = $this->_connection->prepare(
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
            $this->_connection->lastInsertId()
        );
    }

    public function getBookmark(
        ?string $request = null
    ): array
    {
        $query = $this->_connection->prepare(
            'SELECT * FROM `bookmark` WHERE `request` LIKE :request'
        );

        $query->execute(
            [
                ':request' => $request
            ]
        );

        return $query->fetch();
    }

    public function findBookmark(
        ?string $value = null,
        int $start = 0,
        int $limit = 1000
    ): array
    {
        $query = $this->_connection->prepare(
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

    public function deleteBookmark(
        int $id
    ): int
    {
        $query = $this->_connection->query(
            sprintf(
                'DELETE FROM `bookmark` WHERE `id` = %d',
                $id
            )
        );

        return $query->rowCount();
    }

    public function toggleBookmark(
        ?string $request = null,
        ?string $title = null,
        ?int $time = null
    ): bool
    {
        if ($record = $this->getBookmark($request))
        {
            $this->deleteBookmark(
                $record->id
            );

            return false;
        }

        else
        {
            $this->addBookmark(
                $request,
                $title,
                $time
            );

            return true;
        }
    }

    // Cache
    public function addCache(
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

    public function getCache(
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

    public function deleteCache(
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

    public function cleanCache(
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

    public function renewCache(
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
            $this->deleteCache(
                $record->id
            );
        }

        // Add new record
        $this->addCache(
            $request,
            $mime,
            $title,
            $subtitle,
            $tooltip,
            $data,
            $time
        );
    }

    // History
    public function addHistory(
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

    public function findHistory(
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

    public function deleteHistory(
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

    public function cleanHistory(
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

    public function renewHistory(
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

    // Session
    public function addSession(
        ?string $request = null,
        ?int $time = null
    ): int
    {
        $query = $this->_connection->prepare(
            'INSERT INTO `session` (`time`, `request`) VALUES (:time, :request)'
        );

        $query->execute(
            [
                ':time'    => $time ? $time : time(),
                ':request' => $request
            ]
        );

        return intval(
            $this->_connection->lastInsertId()
        );
    }

    public function getSession(): array
    {
        $query = $this->_connection->query(
            'SELECT * FROM `session`'
        );

        if ($session = $query->fetchAll())
        {
            return $session;
        }

        return [];
    }

    public function cleanSession(): int
    {
        $query = $this->_connection->query(
            'DELETE FROM `session`'
        );

        return $query->rowCount();
    }
}