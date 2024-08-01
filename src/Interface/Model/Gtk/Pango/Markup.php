<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model\Gtk\Pango;

/*
 * Pango markup API
 *
 */
interface Markup
{
    public const CODE  = '<tt>%s</tt>';
    public const H1 = '<span size="xx-large">%s</span>';
    public const H2 = '<span size="x-large">%s</span>';
    public const H3 = '<span size="large">%s</span>';
    public const LINK = '<a href="%s" title="%s"><span underline="none">%s</span></a>';
    public const LIST = '<span>%s</span>';
    public const QUOTE = '<i>%s</i>';
    public const TEXT = '<span>%s</span>';

    public const CODE_BEGIN  = '<tt>';
    public const CODE_CLOSE  = '</tt>';

    public const LINK_PREFIX_INTERNAL = ''; // | ⇒
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