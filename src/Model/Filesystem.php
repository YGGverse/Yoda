<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

class Filesystem
{
    private string $_base;

    public function __construct(
        string $dirname
    ) {
        // Unify separators
        $filename = self::_fixDirectorySeparators(
            $dirname
        );

        // Require path
        if (empty($dirname))
        {
            throw new \Exception;
        }

        // Require absolute path
        if (!str_starts_with($dirname, DIRECTORY_SEPARATOR))
        {
            throw new \Exception;
        }

        // Init destination
        if (!is_dir($dirname))
        {
            mkdir(
                $dirname,
                0775,
                true
            );
        }

        // Define filesystem base ending with slash
        $this->_base = realpath(
            $dirname
        ) . DIRECTORY_SEPARATOR;
    }

    public function getBase(): string
    {
        return $this->_base;
    }

    public function getAbsolute(
        string $filename
    ): ?string
    {
        // Require filename
        if (empty($filename))
        {
            throw new \Exception;
        }

        // Unify separators
        $filename = self::_fixDirectorySeparators(
            $filename
        );

        // Check filename is absolute
        if (str_starts_with($filename, DIRECTORY_SEPARATOR))
        {
            // Check absolute filename path started with filesystem base
            if (!str_starts_with($filepath, $this->_base))
            {
                throw new \Exception;
            }

            // Return as is
            return $filename;
        }

        // Append base
        return $this->_base . $filename;
    }

    private static function _fixDirectorySeparators(
        string $path,
        string $separator = DIRECTORY_SEPARATOR
    ): string
    {
        return str_replace(
            [
                '/',
                '\\' // win
            ],
            $separator,
            $path
        );
    }
}