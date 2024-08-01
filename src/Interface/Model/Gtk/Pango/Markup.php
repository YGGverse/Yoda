<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Interface\Model\Gtk\Pango;

/*
 * Pango markup API
 *
 */
interface Markup
{
    public const ENCODING = 'UTF-8';
    public const TAG_CODE = 'tt';
    public const TAG_QUOTE = 'i';
    public const TAG_TEXT = 'span';
    public const TAG_LIST = 'span';
    public const WRAP_WIDTH = 320;

    public static function code(
        string $value
    ): string;

    public static function h1(
        string $value
    ): string;

    public static function h2(
        string $value
    ): string;

    public static function h3(
        string $value
    ): string;

    public static function link(
        string $href,
        string $title,
        string $value
    ): string;

    public static function list(
        string $value
    ): string;

    public static function quote(
        string $value
    ): string;

    public static function text(
        string $value
    ): string;

    public static function pre(
        string $value
    ): string;

    public static function tag(
        string $const,
        bool $close
    ): string;
}