<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Model\Gtk\Pango;

use \GtkLabel;

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
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return sprintf(
            '* %s', // @TODO
            self::_wrap(
                htmlspecialchars(
                    $value
                ),
                $width
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
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            htmlspecialchars(
                $value
            ),
            $width
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

    // @TODO optimization wanted, wordwrap / set_line_wrap not solution
    protected static function _wrap(
        string $string,
        int $width,
        string $break = PHP_EOL,
        int $line = 1,
        array $words = [],
        array $lines = []
    ): string
    {
        $label = new GtkLabel;

        $label->set_use_markup(
            true
        );

        foreach (explode(' ', $string) as $word)
        {
            if (isset($words[$line]))
            {
                $label->set_markup(
                    implode(
                        ' ' , $words[$line]
                    ) . ' ' . $word
                );

                if ($label->get_layout()->get_pixel_size()['width'] > $width)
                {
                    $line++;
                }
            }

            $words[$line][] = $word;
        }

        foreach ($words as $values)
        {
            $lines[] = implode(
                ' ',
                $values
            );
        }

        $label->destroy();

        return implode(
            $break,
            $lines
        );
    }
}