<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model;

use \Yggverse\Yoda\Model\Database;
use \Yggverse\Yoda\Model\Pool;

/*
 * Single async API for multiple protocols
 *
 */
interface Connection
{
    public function __construct(
        Database $database,
        ?Pool $pool = null
    );

    public function request(
        string $request,
        int $timeout = 15
    ): void;

    public function isCompleted(): bool;

    public function setCompleted(
        bool $completed
    ): void;

    public function getTitle(): ?string;

    public function setTitle(
        ?string $title = null
    ): void;

    public function getSubtitle(): ?string;

    public function setSubtitle(
        ?string $subtitle = null
    ): void;

    public function getTooltip(): ?string;

    public function setTooltip(
        ?string $tooltip = null
    ): void;

    public function getMime(): ?string;

    public function setMime(
        ?string $mime = null
    ): void;

    public function getData(): ?string;

    public function setData(
        ?string $data = null
    ): void;

    public function getRedirect(): ?string;

    public function setRedirect(
        ?string $redirect = null
    ): void;

    public function getRequest(): ?array;

    public function setRequest(
        ?string $placeholder,
        bool $visible = true
    ): void;

    public function unsetRequest(): void;

    public function getLength(): ?int;

    public function getCache(
        string $request
    ): ?object;

    public function reset(): void;

    public function close(): void;
}