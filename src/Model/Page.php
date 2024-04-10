<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Page
{
    public static function get(string $name): ?string
    {
        $name = ucfirst(
            mb_strtolower(
                $name
            )
        );

        $filename = __DIR__ .
                    DIRECTORY_SEPARATOR . '..' .
                    DIRECTORY_SEPARATOR . 'Page' .
                    DIRECTORY_SEPARATOR . $name . '.gmi';

        if (file_exists($filename) && is_readable($filename))
        {
            return file_get_contents(
                $filename
            );
        }

        return null;
    }
}