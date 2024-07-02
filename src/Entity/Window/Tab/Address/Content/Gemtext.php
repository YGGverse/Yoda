<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab\Address\Content;

use \Yggverse\Gemtext\Document;
use \Yggverse\Net\Address;

class Gemtext
{
    public \GtkLabel $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab\Address\Content $content;

    // Defaults
    private string $_value = '';

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab\Address\Content $content
    ) {
        $this->content = $content;

        $this->gtk = new \GtkLabel;

        $this->gtk->set_use_markup(
            true
        );

        $this->gtk->set_selectable(
            true
        );

        $this->gtk->set_line_wrap(
            true
        );

        $this->gtk->set_xalign(
            0
        );

        $this->gtk->set_yalign(
            0
        );

        $this->setValue(
            $this->_value
        );

        $this->gtk->connect(
            'activate-link',
            function(
                \GtkLabel $label,
                string $href
            ) {
                $this->content->address->navbar->request->gtk->set_text(
                    $this->_url(
                        $href
                    )
                );

                $this->content->address->update();
            }
        );
    }

    public function setValue(
        string $value,
        string | null &$title = null
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
                        // @TODO multiline
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
                    $this->content->address->navbar->request->gtk->get_text()
                )
            );
        }

        return $address->get();
    }
}