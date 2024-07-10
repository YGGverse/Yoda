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


    public static function getList(
        ?string $dirname,
         string $sort   = 'name',
         int    $order  = SORT_ASC,
         int    $method = SORT_STRING | SORT_NATURAL | SORT_FLAG_CASE
    ): ?array
    {
        // Convert to realpath with ending slash
        if (!$realpath = self::_getRealpath($dirname))
        {
            return null;
        }

        // Make sure requested path is directory
        if (!is_dir($realpath))
        {
            return null;
        }

        // Begin list builder
        $directories = [];
        $files = [];

        foreach ((array) scandir($realpath) as $name)
        {
            // Skip system locations
            if (empty($name) || $name == '.')
            {
                continue;
            }

            // Try to build destination path
            if (!$path = self::_getRealpath($realpath . $name))
            {
                continue;
            }

            // Context
            switch (true)
            {
                case is_dir($path):

                    $directories[] =
                    [
                        'file' => false,
                        'path' => $path,
                        'name' => $name,
                        'link' => urlencode(
                            $name
                        ),
                        'time' => filemtime(
                            $path
                        )
                    ];

                break;

                case is_file($path):

                    $files[] =
                    [
                        'file' => true,
                        'path' => $path,
                        'name' => $name,
                        'link' => urlencode(
                            $name
                        ),
                        'time' => filemtime(
                            $path
                        )
                    ];

                break;
            }
        }

        // Sort order
        array_multisort(
            array_column(
                $directories,
                $sort
            ),
            $order,
            $method,
            $directories
        );

        // Sort files by name ASC
        array_multisort(
            array_column(
                $directories,
                $sort
            ),
            $order,
            $method,
            $directories
        );

        // Merge list
        return array_merge(
            $directories,
            $files
        );
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

    // PHP::realpath extension appending slash to dir paths
    private static function _getRealpath(
        ?string $path
    ): ?string
    {
        if (empty($path))
        {
            return null;
        }

        if (!$realpath = realpath($path))
        {
            return null;
        }

        if (is_dir($realpath))
        {
            $realpath = rtrim(
                $realpath,
                DIRECTORY_SEPARATOR
            ) . DIRECTORY_SEPARATOR;
        }

        return $realpath;
    }
}