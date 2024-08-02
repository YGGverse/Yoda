<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

use \Pdo;

class Database
{
    // Dependencies
    public Pdo $connection;

    // Requirements
    public Database\Auth $auth;
    public Database\Bookmark $bookmark;
    public Database\Cache $cache;
    public Database\History $history;
    public Database\Identity $identity;
    public Database\Session $session;

    public function __construct(
         string $filename,
        ?string $username = null,
        ?string $password = null
    ) {
        // Status
        $exists = file_exists(
            $filename
        );

        // Init dependencies
        $this->connection = new Pdo(
            sprintf(
                'sqlite:%s',
                $filename
            ),
            $username,
            $password
        );

        $this->connection->setAttribute(
            Pdo::ATTR_ERRMODE,
            Pdo::ERRMODE_EXCEPTION
        );

        $this->connection->setAttribute(
            Pdo::ATTR_DEFAULT_FETCH_MODE,
            Pdo::FETCH_OBJ
        );

        // Init requirements
        $this->auth = new Database\Auth(
            $this->connection
        );

        $this->bookmark = new Database\Bookmark(
            $this->connection
        );

        $this->cache = new Database\Cache(
            $this->connection
        );

        $this->history = new Database\History(
            $this->connection
        );

        $this->identity = new Database\Identity(
            $this->connection
        );

        $this->session = new Database\Session(
            $this->connection
        );

        // Init data
        if (!$exists)
        {
            // Open yggverse homepage
            $this->session->add(
                'gemini://yggverse.cities.yesterweb.org/'
            );
        }
    }
}