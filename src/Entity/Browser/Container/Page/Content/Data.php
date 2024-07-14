<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Yggverse\Gemtext\Document;
use \Yggverse\Net\Address;

class Data
{
    public \GtkLabel $gtk;

    // Extras
    public ?string $raw = null;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content $content;

    // Defaults
    private int $_wrap = 140;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page\Content $content
    ) {
        // Init dependency
        $this->content = $content;

        // Init markup label
        $this->gtk = new \GtkLabel;

        $this->gtk->set_use_markup(
            true
        );

        $this->gtk->set_selectable(
            true
        );

        $this->gtk->set_can_focus(
            false
        );

        $this->gtk->set_track_visited_links(
            true
        );

        $this->gtk->set_xalign(
            0
        );

        $this->gtk->set_yalign(
            0
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate-link',
            function(
                \GtkLabel $label,
                string $href
            ) {
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
                if (in_array(
                    parse_url(
                        $url,
                        PHP_URL_SCHEME
                    ),
                    [
                        'nex',
                        'gemini',
                        'file'
                    ])
                ) return true;
            }
        );
    }

    public function setPlain(
        string $value
    ): void
    {
        $this->gtk->set_text(
            $value
        );

        $this->raw = $value;
    }

    public function setMono(
        string $value
    ): void
    {
        $this->gtk->set_markup(
            sprintf(
                '<tt>%s</tt>',
                htmlspecialchars(
                    $value
                )
            )
        );

        $this->raw = $value;
    }

    public function setGemtext(
        string $value,
        string | null &$title = null,
        bool $preformatted = false
    ): void
    {
        $document = new Document(
            $value
        );

        $line = [];

        foreach ($document->getEntities() as $entity)
        {
            switch (true)
            {
                case $entity instanceof \Yggverse\Gemtext\Entity\Code:

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

                case $entity instanceof \Yggverse\Gemtext\Entity\Header:

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

                                throw new \Exception;
                        }
                    }

                break;

                case $entity instanceof \Yggverse\Gemtext\Entity\Link:

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
                            '<a href="%s" title="%s">%s</a>',
                            $this->_url(
                                $entity->getAddress()
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

                case $entity instanceof \Yggverse\Gemtext\Entity\Listing:

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

                case $entity instanceof \Yggverse\Gemtext\Entity\Quote:

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

                case $entity instanceof \Yggverse\Gemtext\Entity\Text:

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

                    throw new \Exception;
            }
        }

        $this->gtk->set_markup(
            implode(
                PHP_EOL,
                $line
            )
        );

        $this->raw = $value;
    }

    private function _wrap(
        string $value
    ): string
    {
        return wordwrap(
            $value,
            $this->_wrap,
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