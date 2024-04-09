<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Memory
{
    private array $_memory = [];

    public function __construct()
    {}

    public function set(string $key, mixed $value): void
    {
        $this->_memory[$key] = $value;
    }

    public function get(string $key): mixed
    {
        if (isset($this->_memory[$key]))
        {
            return $this->_memory[$key];
        }

        return null;
    }
}