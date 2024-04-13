<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class History
{
    private int $_position = -1;

    private array $_record = [];

    public function reset(): void
    {
        $this->_position = -1;

        $this->_record = [];
    }

    public function add(
        string $record
    ): int
    {
        $this->_record[] = $record;

        $this->_position = array_key_last(
            $this->_record
        );

        return $this->_position;
    }

    public function get(
        int $position
    ): ?string
    {
        return isset($this->_record[$position]) ? $this->_record[$position] : null;
    }

    public function getCurrent(): ?string
    {
        return $this->get(
            $this->_position
        );
    }

    public function getBack(): ?string
    {
        return $this->get(
            $this->_position - 1
        );
    }

    public function getForward(): ?string
    {
        return $this->get(
            $this->_position + 1
        );
    }

    public function goBack(): ?string
    {
        $this->_position--;

        return $this->get(
            $this->_position
        );
    }

    public function goForward(): ?string
    {
        $this->_position++;

        return $this->get(
            $this->_position
        );
    }
}