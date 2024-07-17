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

    public function init(
        string $key,
        int $size = 0xfffff,
        string $flags = 'c',
        int $mode = 0644,
    ): ?\Shmop;

    public function read(
        string $key,
        int $start = 0,
        int $count = 0
    ): ?string;

    public function write(
        string $key,
        string $value,
        int $offset = 0
    ): int;

    public function delete(
        string $key
    ): bool;

    public function get(
        string $key
    ): string;

    public function set(
        string $key,
        ?string $value = null
    ): void;

    public function clean(): void;
}