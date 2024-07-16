<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model;

/*
 * Shared memory API for async operations
 *
 */
interface Pool
{
    public function get(
        string $key
    ): mixed;

    public function set(
        string $key,
        mixed $value
    ): void;
}