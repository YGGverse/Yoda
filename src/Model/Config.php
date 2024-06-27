<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Config
{
    public function __construct(
        ?string $filename = null
    ) {
        if (empty($filename))
        {
            $filename = __DIR__ .
                        DIRECTORY_SEPARATOR . '..' .
                        DIRECTORY_SEPARATOR . '..' .
                        DIRECTORY_SEPARATOR . 'config.json';
        }

        if (!file_exists($filename))
        {
            throw new \Exception; // @TODO
        }

        if (!is_readable($filename))
        {
            throw new \Exception; // @TODO
        }

        if (!$data = file_get_contents($filename))
        {
            throw new \Exception; // @TODO
        }

        if (!$config = @json_decode($data))
        {
            throw new \Exception; // @TODO
        }

        foreach ($config as $key => $value)
        {
            $this->{$key} = $value; // @TODO
        }
    }
}