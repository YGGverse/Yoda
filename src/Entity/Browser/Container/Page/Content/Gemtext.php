<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Exception;
use \Gdk;
use \GdkEvent;
use \GtkLabel;
use \Pango;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup;

use \Yggverse\Gemtext\Document;
use \Yggverse\Gemtext\Entity\Code;
use \Yggverse\Gemtext\Entity\Header;
use \Yggverse\Gemtext\Entity\Link;
use \Yggverse\Gemtext\Entity\Listing;
use \Yggverse\Gemtext\Entity\Quote;
use \Yggverse\Gemtext\Entity\Text;

use \Yggverse\Net\Address;

class Gemtext extends Markup
{
    public function set(
        string $source,
        string | null &$title = null,
        bool $preformatted = false
    ): void
    {
        $document = new Document(
            $this->_source = $source
        );

        $line = [];

        foreach ($document->getEntities() as $entity)
        {
            switch (true)
            {
                case $entity instanceof Code:

                    if ($entity->isInline())
                    {
                        $line[] = sprintf(
                            '<tt>%s</tt>',
                            htmlspecialchars(
                                $entity->getAlt()
                            )
                        );
                    }

                    else
                    {
                        $line[] = $preformatted ? '</tt>' : '<tt>';

                        $preformatted = !($preformatted); // toggle
                    }

                break;

                case $entity instanceof Header:

                    if ($preformatted)
                    {
                        $line[] = htmlspecialchars(
                            $this->_wrap(
                                $entity->toString()
                            )
                        );
                    }

                    else
                    {
                        switch ($entity->getLevel())
                        {
                            case 1: // #

                                $line[] = sprintf(
                                    '<span size="xx-large">%s</span>',
                                    htmlspecialchars(
                                        $this->_wrap(
                                            $entity->getText()
                                        )
                                    )
                                );

                                // Find and return document title by first # tag
                                if (empty($title))
                                {
                                    $title = $entity->getText();
                                }

                            break;

                            case 2: // ##

                                $line[] = sprintf(
                                    '<span size="x-large">%s</span>',
                                    htmlspecialchars(
                                        $this->_wrap(
                                            $entity->getText()
                                        )
                                    )
                                );

                            break;

                            case 3: // ###

                                $line[] = sprintf(
                                    '<span size="large">%s</span>',
                                    htmlspecialchars(
                                        $this->_wrap(
                                            $entity->getText()
                                        )
                                    )
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
                        $line[] = htmlspecialchars(
                            $this->_wrap(
                                $entity->toString()
                            )
                        );
                    }

                    else
                    {
                        $line[] = sprintf(
                            '<a href="%s" title="%s"><span underline="none">%s</span></a>',
                            htmlspecialchars(
                                $this->_url(
                                    $entity->getAddress()
                                )
                            ),
                            htmlspecialchars(
                                $entity->getAddress()
                            ),
                            htmlspecialchars(
                                $this->_wrap(
                                    $entity->getAlt() ? $entity->getAlt()
                                                      : $entity->getAddress() // @TODO date
                                )
                            )
                        );
                    }

                break;

                case $entity instanceof Listing:

                    if ($preformatted)
                    {
                        $line[] = htmlspecialchars(
                            $this->_wrap(
                                $entity->toString()
                            )
                        );
                    }

                    else
                    {
                        $line[] = sprintf(
                            '* %s',
                            htmlspecialchars(
                                $this->_wrap(
                                    $entity->getItem()
                                )
                            )
                        );
                    }

                break;

                case $entity instanceof Quote:

                    if ($preformatted)
                    {
                        $line[] = htmlspecialchars(
                            $this->_wrap(
                                $entity->toString()
                            )
                        );
                    }

                    else
                    {
                        $line[] = sprintf(
                            '<i>%s</i>',
                            htmlspecialchars(
                                $this->_wrap(
                                    $entity->getText()
                                )
                            )
                        );
                    }

                break;

                case $entity instanceof Text:

                    if ($preformatted)
                    {
                        $line[] = htmlspecialchars(
                            $this->_wrap(
                                $entity->toString()
                            )
                        );
                    }

                    else
                    {
                        $line[] = htmlspecialchars(
                            $this->_wrap(
                                $entity->getData()
                            )
                        );
                    }

                break;

                default:

                    throw new Exception;
            }
        }

        $this->gtk->set_markup(
            implode(
                PHP_EOL,
                $line
            )
        );
    }

    protected function _onActivateLink(
        GtkLabel $label,
        string $href
    ): bool
    {
        // Format URL
        $url = $this->_url(
            $href
        );

        // Update request entry
        $this->content->page->navbar->request->setValue(
            $this->_url(
                $href
            )
        );

        // Update page
        $this->content->page->update();

        // Prevent propagation for supported protocols
        return in_array(
            parse_url(
                $url,
                PHP_URL_SCHEME
            ),
            [
                'nex',
                'gemini',
                'file'
            ]
        );
    }

    protected function _onButtonPress(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        // Open link in new tab on middle button click
        if ($event->button->button == Gdk::BUTTON_MIDDLE)
        {
            $result = $label->get_layout()->xy_to_index(
                $event->button->x * Pango::SCALE,
                $event->button->y * Pango::SCALE
            );

            if ($result)
            {
                // @TODO
                return true;
            }
        }

        return false;
    }

    protected function _onSizeAllocate(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        // @TODO
        return false;
    }

    private function _wrap(
        string $value
    ): string
    {
        return wordwrap(
            $value,
            $this::WRAP,
            PHP_EOL,
            false
        );
    }

    private function _url(
        string $link
    ): ?string
    {
        $address = new Address(
            $link
        );

        if ($address->isRelative())
        {
            $address->toAbsolute(
                new Address(
                    $this->content->page->navbar->request->getValue()
                )
            );
        }

        return $address->get();
    }
}