<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model;

use \Yggverse\Yoda\Model\Database;
use \Yggverse\Yoda\Model\Pool;

abstract class Connection implements \Yggverse\Yoda\Interface\Model\Connection
{
    private Database $_database;

    private Pool $_pool;

    public function __construct(
        Database $database,
        ?Pool $pool = null
    ) {
        // Init database connection to store cached results
        $this->_database = $database;

        // Use shared memory pool for async operations
        $this->_pool = $pool ? $pool : new Pool;

        // Set defaults
        $this->_pool->init(
            'completed',
            1
        );

        $this->_pool->init(
            'title',
            255
        );

        $this->_pool->init(
            'subtitle',
            255
        );

        $this->_pool->init(
            'tooltip',
            255
        );

        $this->_pool->init(
            'mime',
            32
        );

        $this->_pool->init(
            'data',
            // 1 Mb default
        );

        $this->_pool->init(
            'redirect',
            1024
        );

        $this->_pool->init(
            'request',
            // 1 Mb default
        );
    }

    public function isCompleted(): bool
    {
        return boolval(
            $this->_pool->get(
                'completed'
            )
        );
    }

    public function setCompleted(
        bool $completed
    ): void
    {
        $this->_pool->set(
            'completed',
            strval(
                $completed
            )
        );
    }

    public function getTitle(): ?string
    {
        return $this->_pool->get(
            'title'
        );
    }

    public function setTitle(
        ?string $title = null
    ): void
    {
        $this->_pool->set(
            'title',
            $title
        );
    }

    public function getSubtitle(): ?string
    {
        return $this->_pool->get(
            'subtitle'
        );
    }

    public function setSubtitle(
        ?string $subtitle = null
    ): void
    {
        $this->_pool->set(
            'subtitle',
            $subtitle
        );
    }

    public function getTooltip(): ?string
    {
        return $this->_pool->get(
            'tooltip'
        );
    }

    public function setTooltip(
        ?string $tooltip = null
    ): void
    {
        $this->_pool->set(
            'tooltip',
            $tooltip
        );
    }

    public function getMime(): ?string
    {
        return $this->_pool->get(
            'mime'
        );
    }

    public function setMime(
        ?string $mime = null
    ): void
    {
        $this->_pool->set(
            'mime',
            $mime
        );
    }

    public function getData(): ?string
    {
        return $this->_pool->get(
            'data'
        );
    }

    public function setData(
        ?string $data = null
    ): void
    {
        $this->_pool->set(
            'data',
            $data
        );
    }

    public function getRedirect(): ?string
    {
        return $this->_pool->get(
            'redirect'
        );
    }

    public function setRedirect(
        ?string $redirect = null
    ): void
    {
        $this->_pool->set(
            'redirect',
            $redirect
        );
    }

    public function getRequest(): ?array
    {
        if ($request = $this->_pool->get('request'))
        {
            return unserialize(
                $request
            );
        }

        return null;
    }

    public function setRequest(
        ?string $placeholder,
        bool $visible = true
    ): void
    {
        $this->_pool->set(
            'request',
            serialize(
                [
                    'placeholder' => $placeholder,
                    'visible'     => $visible
                ]
            )
        );
    }

    public function unsetRequest(): void
    {
        $this->_pool->set(
            'request'
        );
    }

    public function getLength(): ?int
    {
        if ($data = $this->_pool->get('data'))
        {
            return mb_strlen(
                $data
            );
        }

        return null;
    }

    public function getCache(
        string $request
    ): ?object
    {
        return $this->_database->cache->get(
            $request
        );
    }

    public function reset(): void
    {
        $this->_pool->reset();
    }

    public function close(): void
    {
        $this->_pool->close();
    }
}