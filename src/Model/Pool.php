<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Pool implements \Yggverse\Yoda\Interface\Model\Pool
{
    private string $_namespace;

    private array $_data = [];

    public function __construct(
        ?string $namespace = null
    ) {
        $this->_namespace = __FILE__ . (
            $namespace ? $namespace : uniqid()
        );
    }

    public function init(
        string $key,
        int $size = 0xfffff,
        string $flags = 'c',
        int $mode = 0644,
    ): ?\Shmop
    {
        if (isset($this->_data[$key]))
        {
            throw new \Exception;
        }

        return $this->_data[$key] = shmop_open(
            crc32(
                $this->_namespace . $key
            ),
            $flags,
            $mode,
            $size
        );
    }

    public function read(
        string $key,
        int $start = 0,
        int $count = 0
    ): ?string
    {
        if (!isset($this->_data[$key]))
        {
            throw new \Exception;
        }

        if (empty($this->_data[$key]))
        {
            return null;
        }

        return shmop_read(
            $this->_data[$key],
            $start,
            $count ? $count : shmop_size(
                $this->_data[$key]
            )
        );
    }

    public function write(
        string $key,
        string $value,
        int $offset = 0
    ): int
    {
        if (!isset($this->_data[$key]))
        {
            throw new \Exception;
        }

        return shmop_write(
            $this->_data[$key],
            $value,
            $offset
        );
    }

    public function delete(
        string $key
    ): bool
    {
        if (!isset($this->_data[$key]))
        {
            throw new \Exception;
        }

        $result = shmop_delete(
            $this->_data[$key]
        );

        $this->_data[$key] = null;

        return $result;
    }

    public function get(
        string $key
    ): string
    {
        return trim(
            strval(
                $this->read(
                    $key
                )
            )
        );
    }

    public function set(
        string $key,
        ?string $value = null
    ): void
    {
        $this->write(
            $key,
            strval(
                $value
            )
        );
    }

    public function clean(): void
    {
        foreach ($this->_data as $key => $shmop)
        {
            $this->delete(
                $key
            );
        }

        $this->_data = [];
    }
}