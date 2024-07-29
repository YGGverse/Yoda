<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Gtk\Pango\Markup;

class Plain extends \Yggverse\Yoda\Abstract\Model\Gtk\Pango\Markup
{
    public static function format(
        string $plain
    ): string
    {
        return self::code(
            $plain
        );
    }
}