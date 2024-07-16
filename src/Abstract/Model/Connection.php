<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model;

abstract class Connection implements \Yggverse\Yoda\Interface\Model\Connection
{
    // Status
    protected bool $_completed = false;

    // Response
    protected ?string $_title = null;
    protected ?string $_subtitle = null;
    protected ?string $_tooltip = null;
    protected ?string $_mime = null;
    protected ?string $_data = null;
    protected ?string $_redirect = null;
    protected ?array  $_request = null;

    public function isCompleted(): bool
    {
        return $this->_completed;
    }

    public function setCompleted(
        bool $completed
    ): void
    {
        $this->_completed = $completed;
    }

    public function getTitle(): ?string
    {
        return $this->_title;
    }

    public function setTitle(
        ?string $title = null
    ): void
    {
        $this->_title = $title;
    }

    public function getSubtitle(): ?string
    {
        return $this->_subtitle;
    }

    public function setSubtitle(
        ?string $subtitle = null
    ): void
    {
        $this->_subtitle = $subtitle;
    }

    public function getTooltip(): ?string
    {
        return $this->_tooltip;
    }

    public function setTooltip(
        ?string $tooltip = null
    ): void
    {
        $this->_tooltip = $tooltip;
    }

    public function getMime(): ?string
    {
        return $this->_mime;
    }

    public function setMime(
        ?string $mime = null
    ): void
    {
        $this->_mime = $mime;
    }

    public function getData(): ?string
    {
        return $this->_data;
    }

    public function setData(
        ?string $data = null
    ): void
    {
        $this->_data = $data;
    }

    public function getRedirect(): ?string
    {
        return $this->_redirect;
    }

    public function setRedirect(
        ?string $redirect = null
    ): void
    {
        $this->_redirect = $redirect;
    }

    public function getRequest(): ?array
    {
        return $this->_request;
    }

    public function setRequest(
        ?array $request = null
    ): void
    {
        $this->_request = $request; // @TODO
    }

    public function getLength(): ?int
    {
        return mb_strlen(
            $this->_data
        );
    }
}