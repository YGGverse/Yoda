<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Gemtext;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Image;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Plain;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Viewport;

use \Yggverse\Yoda\Model\Filesystem;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Dependencies
    public Page $page;

    // Requirements
    public Viewport $viewport;

    // Defaults
    private int $_margin = 8;

    // Extras
    private ?string $_source = null;

    public function __construct(
        Page $page
    ) {
        $this->page = $page;

        // Init scrolled window container
        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        // Init scrolled window viewport
        $this->viewport = new Viewport(
            $this
        );

        $this->gtk->add(
            $this->viewport->gtk
        );
    }

    public function set(
        ?string $mime,
        ?string $data
    ): void
    {
        $this->_source = $data;

        switch ($mime)
        {
            case Filesystem::MIME_TEXT_GEMINI:

                $title = null;

                $document = new Gemtext(
                    $this
                );

                $document->set(
                    $data,
                    $title
                );

                // Update title by gemtext H1 tag (on available)
                if ($title)
                {
                    // Set new title
                    $this->page->title->setValue(
                        $title
                    );

                    // Refresh header by new title if current page is active
                    if ($this->page === $this->page->container->tab->getPage())
                    {
                        $this->page->container->browser->header->setTitle(
                            $this->page->title->getValue(),
                            $this->page->title->getSubtitle()
                        );
                    }
                }

            break;

            case Filesystem::MIME_TEXT_PLAIN:

                $document = new Plain(
                    $this
                );

                $document->set(
                    $data
                );

            break;

            case Filesystem::MIME_IMAGE_GIF:
            case Filesystem::MIME_IMAGE_JPEG:
            case Filesystem::MIME_IMAGE_PNG:
            case Filesystem::MIME_IMAGE_WEBP:

                $document = new Image(
                    $this
                );

                $document->set(
                    $data
                );

            break;

            default:

                $document = new Plain(
                    $this
                );

                $document->set(
                    _('MIME type not supported')
                );
        }

        $this->viewport->set(
            $document
        );

        $this->gtk->show();
    }

    public function getSource(): ?string
    {
        return $this->_source;
    }

    public function refresh()
    {
        // @TODO
    }
}