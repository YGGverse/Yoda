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

    public function get(
        string $key,
        int $start = 0,
        int $count = 0
    ): ?string
    {
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

    public function set(
        string $key,
        ?string $value = null,
        string $flags = 'c',
        int $offset = 0,
        int $mode = 0644,
        ?string $encoding = null
    ): int
    {
        if (empty($value))
        {
            // @TODO delete from memory

            $this->_data[$key] = null;

            return 0;
        }

        if ($this->_data[$key] = shmop_open(crc32($this->_namespace . $key), $flags, $mode, mb_strlen($value, $encoding)))
        {
            return shmop_write(
                $this->_data[$key],
                $value,
                $offset
            );
        }

        throw new \Exception;
    }

    public function reset(): void
    {
        foreach ($this->_data as $data)
        {
            if ($data)
            {
                shmop_delete(
                    $data
                );
            }
        }

        $this->_data = [];
    }
}