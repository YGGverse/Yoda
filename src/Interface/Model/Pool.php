<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model;

/*
 * Shared memory API for async operations
 *
 */
interface Pool
{
    public function __construct(
        ?string $namespace = null
    );

    public function get(
        string $key
    ): ?string;

    public function set(
        string $key,
        ?string $value = null,
        string $flags = 'c',
        int $offset = 0,
        int $mode = 0644,
        ?string $encoding = null
    ): int;

    public function reset(): void;
}