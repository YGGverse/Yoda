<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model\Gtk\Pango;

/*
 * Pango markup API
 *
 */
interface Markup
{
    public const TAG_CODE  = '<tt>%s</tt>';
    public const TAG_CODE_CLOSE  = '</tt>';
    public const TAG_CODE_OPEN  = '<tt>';

    public const TAG_H1 = '<span size="xx-large">%s</span>';
    public const TAG_H2 = '<span size="x-large">%s</span>';
    public const TAG_H3 = '<span size="large">%s</span>';
    public const TAG_LINK = '<a href="%s" title="%s"><span underline="none">%s</span></a>';
    public const TAG_LIST = '<span>%s</span>';
    public const TAG_QUOTE = '<i>%s</i>';
    public const TAG_TEXT = '<span>%s</span>';

    public const LINK_PREFIX_INTERNAL = '⇒';
    public const LINK_PREFIX_EXTERNAL = '⇗';

    public const WRAP_BREAK = PHP_EOL;
    public const WRAP_WIDTH = 320; // px

    public static function code(
        string $value
    ): string;

    public static function h1(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string;

    public static function h2(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string;

    public static function h3(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string;

    public static function link(
        string $href,
        string $title,
        string $value
    ): string;

    public static function list(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string;

    public static function quote(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string;

    public static function text(
        string $value,
        int $width = self::WRAP_WIDTH
    ): string;

    public static function pre(
        string $value
    ): string;
}