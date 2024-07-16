<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model;

use \Yggverse\Yoda\Model\Pool;

abstract class Connection implements \Yggverse\Yoda\Interface\Model\Connection
{
    private Pool $_pool;

    public function __construct(
        ?Pool $pool = null
    ) {
        // Use shared memory pool for async operations
        $this->_pool = $pool ? $pool : new Pool;

        // Set defaults
        $this->_pool->set(
            'completed',
            false
        );

        $this->_pool->set(
            'title',
            null
        );

        $this->_pool->set(
            'subtitle',
            null
        );

        $this->_pool->set(
            'tooltip',
            null
        );

        $this->_pool->set(
            'mime',
            null
        );
        $this->_pool->set(
            'data',
            null
        );

        $this->_pool->set(
            'redirect',
            null
        );

        $this->_pool->set(
            'request',
            null
        );
    }

    public function isCompleted(): bool
    {
        return $this->_pool->get(
            'completed'
        );
    }

    public function setCompleted(
        bool $completed
    ): void
    {
        $this->_pool->set(
            'completed',
            $completed
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
        return $this->_pool->get(
            'request'
        );
    }

    public function setRequest(
        ?string $placeholder,
        bool $visible = true
    ): void
    {
        $this->_pool->set(
            'request',
            [
                'placeholder' => $placeholder,
                'visible'     => $visible
            ]
        );
    }

    public function unsetRequest(): void
    {
        $this->_pool->set(
            'request',
            null
        );
    }

    public function getLength(): ?int
    {
        return mb_strlen(
            $this->_pool->get(
                'data'
            )
        );
    }
}