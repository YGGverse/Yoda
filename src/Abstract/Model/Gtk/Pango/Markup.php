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
            self::CODE,
            self::_escape(
                $value
            )
        );
    }

    public static function h1(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            self::H1,
            $value,
            $width
        );
    }

    public static function h2(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            self::H2,
            $value,
            $width
        );
    }

    public static function h3(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            self::H3,
            $value,
            $width
        );
    }

    public static function link(
        string $href,
        string $title,
        string $value
    ): string
    {
        return sprintf(
            self::LINK,
            self::_escape(
                $href
            ),
            self::_escape(
                $title
            ),
            self::_escape(
                $value
            )
        );
    }

    public static function list(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            self::LIST,
            sprintf(
                '* %s',
                $value
            ),
            $width
        );
    }

    public static function quote(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            self::QUOTE,
            $value,
            $width
        );
    }

    public static function text(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string
    {
        return self::_wrap(
            self::TEXT,
            $value,
            $width
        );
    }

    public static function pre(
        string $value
    ): string
    {
        return self::_escape(
            $value
        );
    }

    protected static function _escape(
        string $value
    ): string
    {
        // @TODO PR #135
        // https://docs.gtk.org/glib/func.markup_escape_text.html
        return htmlspecialchars(
            $value
        );
    }

    // @TODO optimization wanted, wordwrap / set_line_wrap not solution
    protected static function _wrap(
        string $format, // const
        string $value, // unescaped
        int $width = self::WRAP_WIDTH,
        string $break = self::WRAP_BREAK,
        int $line = 1,
        array $words = [],
        array $lines = []
    ): string
    {
        $label = new GtkLabel;

        $label->set_use_markup(
            true
        );

        foreach (explode(' ', $value) as $word)
        {
            if (isset($words[$line]))
            {
                $label->set_markup(
                    sprintf(
                        $format,
                        self::_escape(
                            implode(
                                ' ' , $words[$line]
                            ) . ' ' . $word
                        )
                    )
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

        return sprintf(
            $format,
            self::_escape(
                implode(
                    $break,
                    $lines
                )
            )
        );
    }
}