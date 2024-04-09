<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class File
{
    public static function getConfig(): object
    {
        $filename = __DIR__ .
        DIRECTORY_SEPARATOR . '..' .
        DIRECTORY_SEPARATOR . '..' .
        DIRECTORY_SEPARATOR . 'config.json';

        if (file_exists($filename) && is_readable($filename))
        {
            $result = json_decode(
                file_get_contents(
                    $filename
                )
            );
        }

        if (empty($result))
        {
            throw new \Exception(); // @TODO
        }

        return $result;
    }

    public static function getTheme(string $name): string
    {
        $filename = __DIR__ .
                    DIRECTORY_SEPARATOR . '..' .
                    DIRECTORY_SEPARATOR . 'Theme' .
                    DIRECTORY_SEPARATOR . $name . '.css';

        if (file_exists($filename) && is_readable($filename))
        {
            $result = file_get_contents(
                $filename
            );
        }

        if (empty($result))
        {
            throw new \Exception(); // @TODO
        }

        return $result;
    }
}