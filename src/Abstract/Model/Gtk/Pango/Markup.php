<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model\Gtk\Pango;

class Markup implements \Yggverse\Yoda\Interface\Model\Gtk\Pango\Markup
{
    public static function code(
        string $value
    ): string
    {
        return sprintf(
            '<tt>%s</tt>',
            htmlspecialchars(
                $value
            )
        );
    }

    public static function h1(
        string $value
    ): string
    {
        return sprintf(
            '<span size="xx-large">%s</span>',
            htmlspecialchars(
                $value
            )
        );
    }

    public static function h2(
        string $value
    ): string
    {
        return sprintf(
            '<span size="x-large">%s</span>',
            htmlspecialchars(
                $value
            )
        );
    }

    public static function h3(
        string $value
    ): string
    {
        return sprintf(
            '<span size="large">%s</span>',
            htmlspecialchars(
                $value
            )
        );
    }

    public static function link(
        string $href,
        string $title,
        string $value
    ): string
    {
        return sprintf(
            '<a href="%s" title="%s"><span underline="none">%s</span></a>',
            htmlspecialchars(
                $href
            ),
            htmlspecialchars(
                $title
            ),
            htmlspecialchars(
                $value
            )
        );
    }

    public static function list(
        string $value
    ): string
    {
        return sprintf(
            '* %s', // @TODO
            htmlspecialchars(
                $value
            )
        );
    }

    public static function quote(
        string $value
    ): string
    {
        return sprintf(
            '<i>%s</i>',
            htmlspecialchars(
                $value
            )
        );
    }

    public static function text(
        string $value
    ): string
    {
        return htmlspecialchars(
            $value
        );
    }

    public static function pre(
        string $value
    ): string
    {
        return htmlspecialchars(
            $value
        );
    }

    public static function tag(
        string $const,
        bool $close
    ): string
    {
        if (in_array($const, [self::TAG_CODE]))
        {
            return sprintf(
                $close ? '</%s>' : '<%s>',
                $const
            );
        }

        throw new Exception;
    }
}