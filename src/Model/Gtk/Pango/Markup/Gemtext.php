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
        bool $preformatted = false
    ): string
    {
        $document = new Document(
            $gemtext
        );

        $line = [];

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
                        $line[] = $preformatted ? self::tag(self::TAG_CODE, true)
                                                : self::tag(self::TAG_CODE, false);

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
                                    $entity->getText()
                                );

                                // Find title by first # tag
                                if (is_null($title))
                                {
                                    $title = $entity->getText();
                                }

                            break;

                            case 2: // ##

                                $line[] = self::h2(
                                    $entity->getText()
                                );

                            break;

                            case 3: // ###

                                $line[] = self::h3(
                                    $entity->getText()
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
                        $line[] = self::link(
                            self::_url(
                                $entity->getAddress(),
                                $request
                            ),
                            $entity->getAddress(),
                            $entity->getAlt() ? $entity->getAlt()
                                              : $entity->getAddress() // @TODO date
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
        string $base
    ): ?string
    {
        $address = new Address(
            $link
        );

        if ($address->isRelative())
        {
            $address->toAbsolute(
                new Address(
                    $base
                )
            );
        }

        return $address->get();
    }
}