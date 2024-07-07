<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content;

use \Yggverse\Gemtext\Document;
use \Yggverse\Net\Address;

class Data
{
    public \GtkLabel $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content $content;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content $content
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

        /*
        $this->gtk->set_line_wrap(
            true
        );

        $this->gtk->set_line_wrap_mode(
            @TODO #114
            \GtkWrapMode::WORD
        );
        */

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
                $this->content->page->navbar->request->setValue(
                    $this->_url(
                        $href
                    )
                );

                $this->content->page->update();
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

                    switch ($entity->getLevel())
                    {
                        case 1: // #

                            $line[] = sprintf(
                                '<span size="xx-large">%s</span>',
                                htmlspecialchars(
                                    $entity->getText()
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
                                    $entity->getText()
                                )
                            );

                        break;

                        case 3: // ###

                            $line[] = sprintf(
                                '<span size="large">%s</span>',
                                htmlspecialchars(
                                    $entity->getText()
                                )
                            );

                        break;
                        default:

                            throw new \Exception;
                    }

                break;

                case $entity instanceof \Yggverse\Gemtext\Entity\Link:

                    $line[] = sprintf(
                        '<a href="%s" title="%s">%s</a>',
                        $this->_url(
                            $entity->getAddress()
                        ),
                        htmlspecialchars(
                            $entity->getAddress()
                        ),
                        htmlspecialchars(
                            $entity->getAlt() ? $entity->getAlt()
                                              : $entity->getAddress() // @TODO date
                        )
                    );

                break;

                case $entity instanceof \Yggverse\Gemtext\Entity\Listing:

                    $line[] = sprintf(
                        '* %s',
                        htmlspecialchars(
                            $entity->getItem()
                        )
                    );

                break;

                case $entity instanceof \Yggverse\Gemtext\Entity\Quote:

                    $line[] = sprintf(
                        '<i>%s</i>',
                        htmlspecialchars(
                            $entity->getText()
                        )
                    );

                break;

                case $entity instanceof \Yggverse\Gemtext\Entity\Text:

                    $line[] = htmlspecialchars(
                        $entity->getData()
                    );

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
                    $this->content->page->navbar->request->gtk->get_text()
                )
            );
        }

        return $address->get();
    }
}