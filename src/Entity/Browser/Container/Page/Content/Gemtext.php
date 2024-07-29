<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Exception;
use \Gdk;
use \GdkEvent;
use \GtkLabel;
use \Pango;

use \Yggverse\Yoda\Model\Gtk\Pango\Markup;

use \Yggverse\Gemtext\Document;
use \Yggverse\Gemtext\Entity\Code;
use \Yggverse\Gemtext\Entity\Header;
use \Yggverse\Gemtext\Entity\Link;
use \Yggverse\Gemtext\Entity\Listing;
use \Yggverse\Gemtext\Entity\Quote;
use \Yggverse\Gemtext\Entity\Text;

# use \Yggverse\Gemtext\Parser\Link as LinkParser;

class Gemtext extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup
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
                        $line[] = Markup::code(
                            $entity->getAlt()
                        );
                    }

                    else
                    {
                        $line[] = $preformatted ? Markup::tag(Markup::TAG_CODE, true)
                                                : Markup::tag(Markup::TAG_CODE, false);

                        $preformatted = !($preformatted); // toggle
                    }

                break;

                case $entity instanceof Header:

                    if ($preformatted)
                    {
                        $line[] = Markup::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        switch ($entity->getLevel())
                        {
                            case 1: // #

                                $line[] = Markup::h1(
                                    $entity->getText()
                                );

                                // Find and return document title by first # tag
                                if (empty($title))
                                {
                                    $title = $entity->getText();
                                }

                            break;

                            case 2: // ##

                                $line[] = Markup::h2(
                                    $entity->getText()
                                );

                            break;

                            case 3: // ###

                                $line[] = Markup::h3(
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
                        $line[] = Markup::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = Markup::h3(
                            $this->_url(
                                $entity->getAddress()
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
                        $line[] = Markup::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = Markup::list(
                            $entity->getItem()
                        );
                    }

                break;

                case $entity instanceof Quote:

                    if ($preformatted)
                    {
                        $line[] = Markup::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = Markup::quote(
                            $entity->getText()
                        );
                    }

                break;

                case $entity instanceof Text:

                    if ($preformatted)
                    {
                        $line[] = Markup::pre(
                            $entity->toString()
                        );
                    }

                    else
                    {
                        $line[] = Markup::text(
                            $entity->getData()
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
        // Open link in the new tab on middle button click
        if ($event->button->button == Gdk::BUTTON_MIDDLE)
        {
            // Detect cursor position
            $result = $label->get_layout()->xy_to_index(
                $event->button->x * Pango::SCALE,
                $event->button->y * Pango::SCALE
            );

            // Position detected
            if ($result)
            {
                // Get entire line from source

                /* @TODO incorrect offset index_
                if ($line = $this->_line($result['index_']))
                {
                    // Parse gemtext href
                    if ($href = LinkParser::getAddress($line))
                    {
                        // Format URL
                        if ($url = $this->_url($href))
                        {
                            // Open
                            $this->content->page->container->tab->append(
                                $url,
                                true,
                                false
                            );

                            return true;
                        }
                    }
                } */
            }
        }

        return false;
    }
}