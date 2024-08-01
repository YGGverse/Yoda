<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Gtk\Pango\Markup;

use \Yggverse\Gemtext\Document;
use \Yggverse\Gemtext\Entity\Code;
use \Yggverse\Gemtext\Entity\Header;
use \Yggverse\Gemtext\Entity\Link;
use \Yggverse\Gemtext\Entity\Listing;
use \Yggverse\Gemtext\Entity\Quote;
use \Yggverse\Gemtext\Entity\Text;
use \Yggverse\Net\Address;

class Gemtext extends \Yggverse\Yoda\Abstract\Model\Gtk\Pango\Markup
{
    public static function format(
        string $gemtext,
        string $request,
        int $width = self::WRAP_WIDTH,
        ?string &$title = null,
        bool $preformatted = false,
        array $line = []
    ): string
    {
        $document = new Document(
            $gemtext
        );

        foreach ($document->getEntities() as $entity)
        {
            switch (true)
            {
                case $entity instanceof Code:

                    if ($entity->isInline())
                    {
                        $line[] = self::code(
                            $entity->getAlt()
                        );
                    }

                    else
                    {
                        $line[] = $preformatted ? self::CODE_CLOSE
                                                : self::CODE_BEGIN;

                        $preformatted = !($preformatted); // toggle
                    }

                break;

                case $entity instanceof Header:

                    if ($preformatted)
                    {
                        $line[] = self::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        switch ($entity->getLevel())
                        {
                            case 1: // #

                                $line[] = self::h1(
                                    $entity->getText(),
                                    $width
                                );

                                // Find title by first # tag
                                if (is_null($title))
                                {
                                    $title = $entity->getText();
                                }

                            break;

                            case 2: // ##

                                $line[] = self::h2(
                                    $entity->getText(),
                                    $width
                                );

                            break;

                            case 3: // ###

                                $line[] = self::h3(
                                    $entity->getText(),
                                    $width
                                );

                            break;
                            default:

                                throw new Exception;
                        }
                    }

                break;

                case $entity instanceof Link:

                    if ($preformatted)
                    {
                        $line[] = self::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $prefix = self::LINK_PREFIX_EXTERNAL;

                        $line[] = self::link(
                            self::_url(
                                $entity->getAddress(),
                                $request,
                                $prefix
                            ),
                            $entity->getAddress(),
                            trim(
                                implode(
                                    ' ',
                                    [
                                        $prefix,
                                        $entity->getDate(),
                                        $entity->getAlt() ? $entity->getAlt() : $entity->getAddress()
                                    ]
                                )
                            )
                        );
                    }

                break;

                case $entity instanceof Listing:

                    if ($preformatted)
                    {
                        $line[] = self::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = self::list(
                            $entity->getItem(),
                            $width
                        );
                    }

                break;

                case $entity instanceof Quote:

                    if ($preformatted)
                    {
                        $line[] = self::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = self::quote(
                            $entity->getText(),
                            $width
                        );
                    }

                break;

                case $entity instanceof Text:

                    if ($preformatted)
                    {
                        $line[] = self::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = self::text(
                            $entity->getData(),
                            $width
                        );
                    }

                break;

                default:

                    throw new Exception;
            }
        }

        return implode(
            PHP_EOL,
            $line
        );
    }

    private static function _url(
        string $link,
        string $base,
        string &$prefix = self::LINK_PREFIX_EXTERNAL
    ): ?string
    {
        $address = new Address(
            $link
        );

        $request = new Address(
            $base
        );

        if ($address->isRelative())
        {
            $address->toAbsolute(
                $request
            );
        }

        $prefix = $address->getScheme() == $request->getScheme() ? self::LINK_PREFIX_INTERNAL
                                                                 : self::LINK_PREFIX_EXTERNAL;

        return $address->get();
    }
}