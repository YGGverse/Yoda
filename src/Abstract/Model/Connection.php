<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model;

use \Yggverse\Yoda\Model\Buffer;

abstract class Connection implements \Yggverse\Yoda\Interface\Model\Connection
{
    private Buffer $_buffer;

    public function __construct(
        Buffer $buffer
    ) {
        // Use shared memory for async operations
        $this->_buffer = $buffer;

        // Set defaults
        $this->_buffer->set(
            'completed',
            false
        );

        $this->_buffer->set(
            'title',
            null
        );

        $this->_buffer->set(
            'subtitle',
            null
        );

        $this->_buffer->set(
            'tooltip',
            null
        );

        $this->_buffer->set(
            'mime',
            null
        );
        $this->_buffer->set(
            'data',
            null
        );

        $this->_buffer->set(
            'redirect',
            null
        );

        $this->_buffer->set(
            'request',
            null
        );
    }

    public function isCompleted(): bool
    {
        return $this->_buffer->get(
            'completed'
        );
    }

    public function setCompleted(
        bool $completed
    ): void
    {
        $this->_buffer->set(
            'completed',
            $completed
        );
    }

    public function getTitle(): ?string
    {
        return $this->_buffer->get(
            'title'
        );
    }

    public function setTitle(
        ?string $title = null
    ): void
    {
        $this->_buffer->set(
            'title',
            $title
        );
    }

    public function getSubtitle(): ?string
    {
        return $this->_buffer->get(
            'subtitle'
        );
    }

    public function setSubtitle(
        ?string $subtitle = null
    ): void
    {
        $this->_buffer->set(
            'subtitle',
            $subtitle
        );
    }

    public function getTooltip(): ?string
    {
        return $this->_buffer->get(
            'tooltip'
        );
    }

    public function setTooltip(
        ?string $tooltip = null
    ): void
    {
        $this->_buffer->set(
            'tooltip',
            $tooltip
        );
    }

    public function getMime(): ?string
    {
        return $this->_buffer->get(
            'mime'
        );
    }

    public function setMime(
        ?string $mime = null
    ): void
    {
        $this->_buffer->set(
            'mime',
            $mime
        );
    }

    public function getData(): ?string
    {
        return $this->_buffer->get(
            'data'
        );
    }

    public function setData(
        ?string $data = null
    ): void
    {
        $this->_buffer->set(
            'data',
            $data
        );
    }

    public function getRedirect(): ?string
    {
        return $this->_buffer->get(
            'redirect'
        );
    }

    public function setRedirect(
        ?string $redirect = null
    ): void
    {
        $this->_buffer->set(
            'redirect',
            $redirect
        );
    }

    public function getRequest(): ?array
    {
        return $this->_buffer->get(
            'request'
        );
    }

    public function setRequest(
        ?string $placeholder,
        bool $visible = true
    ): void
    {
        $this->_buffer->set(
            'request',
            [
                'placeholder' => $placeholder,
                'visible'     => $visible
            ]
        );
    }

    public function unsetRequest(): void
    {
        $this->_buffer->set(
            'request',
            null
        );
    }

    public function getLength(): ?int
    {
        return mb_strlen(
            $this->_buffer->get(
                'data'
            )
        );
    }
}